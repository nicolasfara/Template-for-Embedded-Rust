# STM32 Embassy Rust Template

Template repository for `no_std` embedded Rust firmware on STM32 microcontrollers with
[Embassy](https://embassy.dev/). The default configuration targets the
Nucleo-F401RE board (`stm32f401re`) and runs a small async hello world that blinks
the user LED on `PA5` while logging with `defmt` over RTT.

After pushing this repository to GitHub, enable **Template repository** in the
repository settings so new projects can be generated from it.

## Getting Ready

Install Rust and the embedded target:

```sh
rustup target add thumbv7em-none-eabihf
rustup component add clippy rustfmt llvm-tools-preview
```

Install the runner used by `.cargo/config.toml`:

```sh
cargo install probe-rs-tools --locked
```

Optional tools used by CI:

```sh
cargo install cargo-deny --locked
```

Build the default firmware:

```sh
cargo build --release
```

Flash and run it on a Nucleo-F401RE with an ST-Link/debug probe attached:

```sh
cargo run --release
```

You should see RTT logs from `defmt` and the board LED should blink.

## Selecting An STM32 Chip

The chip is selected with Cargo features. The template enables `stm32f401re` by
default:

```sh
cargo build --release --no-default-features --features stm32f401re
```

To add another STM32 IC, add a crate feature in `Cargo.toml` that forwards to the
matching `embassy-stm32/<chip>` feature. If the new chip uses a different ARM
core or FPU configuration, also update `.cargo/config.toml`, `rust-toolchain.toml`,
and CI to use the correct Rust target triple and `probe-rs` chip name.

## Repository Structure

- `src/main.rs`: Embassy async hello world for the default board.
- `.cargo/config.toml`: target triple, linker arguments, `probe-rs` runner, and
  default `DEFMT_LOG`.
- `.github/workflows/ci.yml`: formatting, linting, dependency policy, and firmware
  build checks.
- `.github/workflows/release.yml`: Conventional Commit release automation with
  firmware artifacts.
- `Cargo.toml`, `rustfmt.toml`, `clippy.toml`, `deny.toml`: dependency, lint, and
  policy configuration.

## Quality Checks

Run the same checks as CI:

```sh
cargo fmt --all --check
cargo clippy --locked --bins --no-default-features --features stm32f401re -- -D warnings
cargo check --locked --release --no-default-features --features stm32f401re
cargo deny check
```

## Releases

Releases are driven by Conventional Commits on the `main` branch through Release
Please:

```text
feat: add a new board feature
fix: correct the LED pin for a board
feat!: change the default chip family
```

When Release Please creates a GitHub release, the release workflow builds the
default firmware and uploads both `.elf` and `.bin` artifacts.
