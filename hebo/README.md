
[![](resources/logo/hebo.png)](https://github.com/RustRobotics/hebo)

# Hebo
![Build status](https://github.com/RustRobotics/hebo/actions/workflows/rust.yml/badge.svg)
[![Latest version](https://img.shields.io/crates/v/hebo.svg)](https://crates.io/crates/hebo)
[![Documentation](https://docs.rs/hebo/badge.svg)](https://docs.rs/hebo)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.56+-yellow.svg)
![License](https://img.shields.io/crates/l/hebo.svg)

HeBo (河伯) is a distributed MQTT broker in Rust.

- [Documentation](https://docs.rs/hebo)
- [Release notes](https://github.com/RustRobotics/hebo/releases)

## Build on Linux
First install dependencies:
```bash
sudo apt install -y \
  gcc \
  libssl-dev \
  libhiredis-dev \
  libmongoc-dev \
  libmariadb-dev \
  libpq-dev
```

## Build on Windows
Install [precompiled openssl](https://slproweb.com/products/Win32OpenSSL.html) is the easiest way.

For more information, 
see [openssl document](https://docs.rs/crate/openssl-sys/0.9.19)

## License
This project is Licensed under [GNU General Public License, version 3.0](LICENSE).
