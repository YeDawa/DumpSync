# Installation

## Installing Rust on Different Operating Systems

#### 1. Linux and macOS

To install Rust on **Linux** and **macOS**, use the `rustup` script:

1. Open your terminal.
2. Run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Follow the on-screen instructions to complete the installation.
4. After the installation, restart the terminal and verify it by running:

```bash
rustc --version
```

---

#### 2. Windows

To install Rust on **Windows**, follow these steps:

1. Download the `rustup-init.exe` installer from the official website: [https://rust-lang.org/tools/install](https://rust-lang.org/tools/install).
2. Run the installer and follow the setup instructions.
3. After installation, open **Command Prompt** or **PowerShell** and verify Rust with:

```powershell
rustc --version
```

**Note:** Rust will be added to your PATH automatically during installation.

---

## Installing DumpSync in your Operational System

Once Rust is installed, use this command to install **DumpSync**:

```bash
cargo install dumpsync
```

This will install **DumpSync** globally, allowing you to run it from any directory.