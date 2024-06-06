# Pico W Rust Starter Kit with Debug Probe

This is a kit on how to use `Rust` with the **Raspberry Pi Pico W**. 

Please change `src/config.rs`:
```Rust
pub const WIFI_SSID: &str = "YOUR_WIFI_SSID";
pub const WIFI_PASSWORD: &str = "YOUR_WIFI_PASSWORD";
```

## Linux

```Bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup default nightly
rustup target add thumbv6m-none-eabi

# Embassy
git clone https://github.com/embassy-rs/embassy.git embassy

# USB Permissions
sudo usermod -aG dialout $USER
sudo cp 69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload
sudo udevadm trigger

# Debugger
sudo apt-get install gdb-arm-none-eabi openocd socat
cargo install defmt-print
cargo install elf2uf2-rs
chmod +x linux-*
```

### Linux: Running Everything

Open three terminals.

**Terminal 1**

    ./linux-debugger-openocd.sh

**Terminal 2**

    ./linux-debugger-gdb.sh

**Terminal 3**

    ./linux-debugger-serial.sh

## Windows

Open **Powershell** via `Run as Administrator`

```powershell
Set-ExecutionPolicy Bypass -Scope Process

# Install Chocolatey
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
choco install putty
```
Download the latest version of Pico SDK: https://github.com/raspberrypi/pico-setup-windows/releases/latest/download/pico-setup-windows-x64-standalone.exe

Also edit the scripts `windows-debugger-gdb.ps1` `windows-debugger-openocd.ps1` from `C:\Program Files\Raspberry Pi\Pico SDK v1.5.1\` to whatever is the newest version.

Install Rust Up https://win.rustup.rs/x86_64. Then Install dependencies. 

```bash
# Install Rust Deps
rustup default nightly
rustup target add thumbv6m-none-eabi

# Install Cargo Tools
cargo install defmt-print
cargo install elf2uf2-rs

# Embassy
git clone https://github.com/embassy-rs/embassy.git embassy
```

### Windows: Running Everything

Open three terminals.

**Terminal 1**

    ./windows-debugger-openocd.ps1

**Terminal 2**

    ./windows-debugger-gdb.ps1

**Terminal 3**

Make sure you change "COM5" to the COM ports of your debug probe.

    ./windows-debugger-serial.ps1