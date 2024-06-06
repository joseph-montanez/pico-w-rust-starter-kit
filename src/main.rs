#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(panic_info_message)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]

mod config;

use core::str::from_utf8;
use cyw43_pio::PioSpi;
use defmt::{info, unwrap, warn};
use defmt_serial as _;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::{bind_interrupts, uart};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0, UART0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::uart::Uart;
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use static_cell::{make_static, StaticCell};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    UART0_IRQ => uart::InterruptHandler<UART0>;
});

static UART: StaticCell<Uart<'static, UART0, uart::Blocking>> = StaticCell::new();

const WIFI_NETWORK: &str = config::WIFI_SSID;
const WIFI_PASSWORD: &str = config::WIFI_PASSWORD;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        defmt::error!("Panic occurred at file '{}', line {}", location.file(), location.line());
    } else {
        defmt::error!("Panic occurred!");
    }
    loop {}
}

#[embassy_executor::task(pool_size = 2)]
async fn wifi_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::task(pool_size = 2)]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    // UART setup
    let uart = uart::Uart::new_with_rtscts_blocking(
        p.UART0, p.PIN_0, p.PIN_1, p.PIN_3, p.PIN_2, uart::Config::default()
    );

    let uart_ref = UART.init(uart);
    defmt_serial::defmt_serial(uart_ref);

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(&mut pio.common, pio.sm0, pio.irq0, cs, p.PIN_24, p.PIN_29, p.DMA_CH0);

    info!("creating cyw43...");
    let state = make_static!(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(wifi_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());

    // Generate random seed
    let seed = 0x0123_4567_89ab_cdef; // chosen by fair dice roll. guaranteed to be random.

    // Init network stack
    let stack = &*make_static!(Stack::new(
        net_device,
        config,
        make_static!(StackResources::<2>::new()),
        seed
    ));

    unwrap!(spawner.spawn(net_task(stack)));


    info!("joining network...");
    loop {
        match control.join_wpa2(WIFI_NETWORK, WIFI_PASSWORD).await {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
            }
        }
    }

    // Wait for DHCP, not necessary when using static IP
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up!");

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];

    control.gpio_set(0, false).await;
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(Duration::from_secs(10)));

    info!("Listening on TCP:8000...");
    control.gpio_set(0, true).await;

    loop {
        info!("Waiting for connection...");

        if let Err(e) = socket.accept(8000).await {
            warn!("accept error: {:?}", e);
            core::mem::drop(socket);

            socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
            socket.set_timeout(Some(Duration::from_secs(10)));
            continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    warn!("connection read error: {:?}", e);
                    continue;
                },
            };

            if n > 0 {
                info!("rxd {}", from_utf8(&buf[..n]).unwrap());
            } else {
                info!("rxd has no information");
            }

            let http_response = "HTTP/1.0 200 OK\r\nContent-Type: text/html\r\n\r\nHello";

            match socket.write_all(&http_response.as_bytes()).await {
                Ok(()) => {
                    socket.close();
                }
                Err(e) => {
                    warn!("write error: {:?}", e);
                    break;
                }
            };
        }
    }
}