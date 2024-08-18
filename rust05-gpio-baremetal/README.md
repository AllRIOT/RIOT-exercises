# Application example outside of RIOT

The example in `./blink/` uses embedded Rust without any operating system.

The files `Cargo.toml`, `memory.x` and `.cargo/config.toml` set up the embedded Rust toolchain so that you can build and flash it on a BBC micro:bit v2 with:

```sh
$ cd blink/
$ cargo run
```

## Task 1

* **1. Take the portable blinking function of the rust05-gpio example task 3,**
  **copy it into `blink/src/main.rs` and call it from the main function.**

* **2. Flash the example:**

```sh
$ cargo run
```
