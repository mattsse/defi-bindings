//! Generates the bindings
use foundry_binder::{Binder, RepositoryBuilder, Url};

// github repository url
const REPO_URL: &str = "<the-url-of-the-project>";

// the release tag for which to generate bindings for
const RELEASE_TAG: &str = "v3.0.0";

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
