[![Crate](https://img.shields.io/crates/v/libnotcurses-sys.svg)](https://crates.io/crates/libnotcurses-sys)
[![API](https://docs.rs/libnotcurses-sys/badge.svg)](https://docs.rs/libnotcurses-sys/)
[![MSRV: 1.64.0](https://flat.badgen.net/badge/MSRV/1.64.0/purple)](https://releases.rs/docs/released/1.64.0/)

`libnotcurses-sys` is a low-level Rust wrapper for the
[notcurses C library](https://www.github.com/dankamongmen/notcurses/)

It's recommended to use the [notcurses higher level bindings][notcurses-rs].

[notcurses-rs]: https://crates.io/crates/notcurses

## Example

```rust
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    let stdplane = unsafe { nc.stdplane() };
    stdplane.putstr("\nhello world!\n")?;
    nc.render()?;
    unsafe { nc.stop()? };
    Ok(())
}
```

## Versioning

The current version is compatible with notcurses [`3.0.9`][tag].

*Current major version `3` is considered a development version*.

[tag]: https://github.com/dankamongmen/notcurses/releases/tag/v3.0.9
