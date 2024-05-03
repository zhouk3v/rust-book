Cargo has two main build profiles: `dev` and `release`

- `dev` is the profile Cargo uses when running `cargo build`
- `release` is the profile used when running `cargo build --release`

You can customize build profiles in the `Cargo.toml` file, by adding `[profile.*]` sections:

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

```

One example is the `opt-level` setting, which controls the number of optimizations Rust will apply to your code

- `opt-level` has a range of 0 to 3, which applies more optimizations as the level increases
- `dev` uses a default level of 0 to mimimize compile times, since development builds are compiled many times
- `release` uses a default level of 3 since releases are compiled once and ran many times
