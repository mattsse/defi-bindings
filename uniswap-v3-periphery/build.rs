//! Generates the bindings
use foundry_binder::{Binder, RepositoryBuilder, Url};

// github repository url
const REPO_URL: &str = "https://github.com/Uniswap/v3-periphery";

// the release tag for which to generate bindings for
const RELEASE_TAG: &str = "v1.3.0";

fn generate() {
    let binder =
        Binder::new(RepositoryBuilder::new(Url::parse(REPO_URL).unwrap()).tag(RELEASE_TAG));

    binder.generate().expect("Failed to generate bindings")
}

fn main() {
    // only generate if `FRESH_BINDINGS` env var is set
    if std::env::var("FRESH_BINDINGS").is_ok() {
        generate()
    }
}
