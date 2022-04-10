# defi-bindings

Contains various [ethers-rs](https://github.com/gakonst/ethers-rs) rust bindings of defi projects generated with [foundry-binder](https://github.com/gakonst/foundry/tree/master/binder).

## Use the bindings

Import directly from this repo

```toml
uniswap-bindings = { git = "https://github.com/mattsse/defi-bindings" }
```

## Generate your own

You can the [`./template`](template) crate as base.

which has following deps in its `Cargo.toml`

```toml
[dependencies]
ethers = { git = "https://github.com/gakonst/ethers-rs", features = ["abigen"] }
serde_json = "1.0"

[build-dependencies]
foundry-binder = { git = "https://github.com/gakonst/foundry/tree/master/binder" }
```

In the `build.rs` file, you must configure which project you want to check out, build, and generate bindings for.

```rust
use foundry_binder::{Binder, RepositoryBuilder, Url};

// github repository url
const REPO_URL: &str = "<the-url-of-the-project>";

// the release tag for which to generate bindings for
const RELEASE_TAG: &str = "v3.0.0";

/// This clones the project, builds the project and generates rust bindings
fn generate() {
    let binder =
        Binder::new(RepositoryBuilder::new(Url::parse(REPO_URL).unwrap())
            // generate bindings for this release tag
            // if not set, then the default branch will be used
            .tag(RELEASE_TAG))
            // keep build artifacts in `artifacts` forlder
            .keep_artifacts("artifacts");

    binder.generate().expect("Failed to generate bindings")
}

fn main() {
    // only generate if `FRESH_BINDINGS` env var is set
    if std::env::var("FRESH_BINDINGS").is_ok() {
        generate()
    }
}
```

To build the project, simply run `cargo build` which executes the `build.rs`. Afterwards, if everything succeeded, the bindings are written to `src/contracts`.
The `FRESH_BINDINGS` ensures that these steps are not executed on each new `cargo build`.

```shell
FRESH_BINDINGS="" cargo build
```


## License

Licensed under either of these:

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)
   
