# coin-build-tools

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

`coin-build-tools` provides a set of tools to build [Coin-OR] libraries from source.

## Usage

Just add the following to your `Cargo.toml`:

```toml
[build-dependencies]
coin-build-tools = "\*"
```

## Configuration

### Environment

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_${LIB_NAME}_STATIC` to link to CoinUtils statically;
* `CARGO_${LIB_NAME}_SYSTEM` to link to CoinUtils system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

## Windows and vcpkg

On Windows, if `${LIB_NAME}_SYSTEM` is set to `1`, `osi-src` will use
[vcpkg] to find Osi. Before building, you must have the correct Osi
installed for your target triplet and kind of linking. For instance,
to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install
 `osi` for the `x64-windows` triplet:

```sh
vcpkg install osi --triplet x64-windows
```

To link Osi statically, install `osi` for the `x64-windows-static-md` triplet:

```sh
vcpkg install osi --triplet x64-windows-static-md
```

To link Osi and C Runtime (CRT) statically, install `osi` for the `x64-windows-static` triplet:

```sh
vcpkg install osi --triplet x64-windows-static
```

and build with `+crt-static` option

```sh
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

you can use it for the other target by providing the `--target` option to
`cargo build`.

| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `aarch64-unknown-linux-gnu`          | ✓   |
| `aarch64-unknown-linux-musl`         | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-msvc`             | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |
| `x86_64-unknown-linux-musl`          | ✓   |
| others                               | not test   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/coin-build-tools/badge.svg
[documentation-url]: https://docs.rs/coin-build-tools
[package-img]: https://img.shields.io/crates/v/coin-build-tools.svg
[package-url]: https://crates.io/crates/coin-build-tools
[license-img]: https://img.shields.io/crates/l/coin-build-tools.svg
[license-url]: https://github.com/Maroon502/coin-build-tools/blob/master/LICENSE.md