//! Generates the bindings
use foundry_binder::{Binder, RepositoryBuilder, Url};

const REPO_URL: &str = "https://github.com/maple-labs/loan";

const RELEASE_TAG: &str = "v3.0.0";

fn generate() {
    let binder =
        Binder::new(RepositoryBuilder::new(Url::parse(REPO_URL).unwrap()).tag(RELEASE_TAG));

    binder.generate().expect("Failed to generate bindings")
}

fn main() {
    if std::env::var("FRESH_BINDINGS").is_ok() {
        generate()
    }
}
