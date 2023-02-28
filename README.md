# `rttmon`

> A simple RTT monitor for [OpenOCD](https://openocd.org)

[![crates.io](https://img.shields.io/crates/v/rttmon)](https://crates.io/crates/rttmon)
[![docs.rs](https://img.shields.io/docsrs/rttmon)](https://docs.rs/rttmon)
[![github](https://img.shields.io/github/actions/workflow/status/matteocarnelos/rttmon/cargo.yml?branch=main)](https://github.com/matteocarnelos/rttmon/actions/workflows/cargo.yml)

## Features

- Print RTT messages with (host) timestamps
- Automatic reconnection upon OpenOCD server restart

## Installation

```
cargo install rttmon
```

## Usage

```
rttmon [HOST] [PORT]
```

#### Arguments

- `[HOST]`: The OpenOCD server host [defualt: `localhost`]
- `[PORT]`: The OpenOCD server port [default: `9090`]

For more usage info, run `rttmon --help`.

#### Additional resources

- [OpenOCD RTT Commands](https://openocd.org/doc/html/General-Commands.html#Real-Time-Transfer-_0028RTT_0029)

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
