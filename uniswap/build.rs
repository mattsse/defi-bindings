//! Generates the bindings
use foundry_binder::Binder;

fn main() {
    let binder = Binder::new(
        "/Users/Matthias/git/rust/foundry-integration-tests/testdata/dapptools-template",
    );

    binder.generate().expect("Failed to generate bindings")
}
