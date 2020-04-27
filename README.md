# nightlight

> A CLI for configuring "Night Shift" on macOS 🌕🌖🌗🌘🌑

This crate also doubles as a Rust library. 🦀

### Why?

The "Night Shift" feature on macOS is a convenient, built-in feature
that can theoretically accomplish most of what third-party alternatives
(like [f.lux®](https://justgetflux.com/)) are capable of. However, as
of now, there is no way to programmatically configure Night Shift (without
entering the system preferences GUI), making its current usage more limited.

This `nightlight` CLI aims to enable such access via a few simple commands.
(Or, alternatively, via library access for other Rust tools.)

## Installing

[Set up Rust/Cargo](https://doc.rust-lang.org/book/ch01-01-installation.html)
and install from crates.io by running:

```
cargo install nightlight
```

Or clone this repo and install from the local path:

```
cargo install --path .
```

## Usage

First, make sure you are running macOS Sierra or newer.

### Command-Line Interface

Turn Night Shift on (until tomorrow/sunrise):

```
nightlight on
```

Turn Night Shift off:

```
nightlight off
```

Set color temperature (a number from 0 to 100):

```
nightlight temp 70
```

Schedule from sunset to sunrise:

```
nightlight schedule
```

Set a custom schedule (in 12 or 24-hour time format):

```
nightlight schedule 19:45 6:00
nightlight schedule 7:45pm 6am
```

Disable the current schedule:

```
nightlight unschedule
```

View current schedule, on/off state, and color temperature preference:

```
nightlight status
```

### Rust API

In addition to a CLI, `nightlight` can be pulled-in as a dependency for other Rust crates:

```
nightlight = "0.0.4"
```

Here's an example `fn` that toggles Night Shift off,
changes the schedule and color temperature preference,
and then toggles the feature back on:

```rust
extern crate nightlight;

use nightlight::{NightShift, Schedule};

fn main() {
    let night_shift = nightlight::new();

    if night_shift.status().unwrap().currently_active {
        println!("Turning Night Shift off...");
        night_shift.off().unwrap();
    }

    println!("Setting schedule and temperature...");
    night_shift.set_schedule(Schedule::SunsetToSunrise).unwrap();
    night_shift.set_temp(70).unwrap();

    println!("Turning Night Shift on...");
    night_shift.on().unwrap();
}
```

## Todo:

- [X] Ability to see current status of Night Shift
- [X] Ability to enable/disable sunrise/sundown schedule
- [X] Ability to enable/disable custom schedules
- [ ] Ensure that changing schedule doesn't affect on/off state.
- [ ] API improvements and full documentation
- [ ] Test coverage of schedule/time parsing.
- [ ] Tests that use fake/stub ObjC library.
- [ ] Cross-platform support (e.g. Windows' "Night Light")

## Contributing

* Check the issue tracker and consider creating a new issue.
* Fork the project and create a new branch for your contribution.
* Write, commit, and push your contribution to your branch.
* Make sure the project builds (`cargo build`) and functionality still works as expected.
* Submit a pull request.

## Thanks To:

* The team at Apple for introducing this feature in macOS Sierra
* GitHub user `jenghis` for the (now archived) [nshift](https://github.com/jenghis/nshift) repo/CLI
* The maintainers of the Rust [objc crate](https://github.com/SSheldon/rust-objc)
* Carol Nichols and Steve Klabnik for the [official book](https://doc.rust-lang.org/book/) on Rust

## License

`nightlight` is released under the [MIT License](LICENSE).
