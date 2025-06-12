# Releasing

Releasing a new version needs a few steps that we want to automate, yet are currently manual:

## Update the version in file `Cargo.toml`

Edit the file `Cargo.toml`.

Update the version number.

## Verify

Verify everything is correct locally:

```sh
cargo test
cargo build --release
cargo doc
```

## Push

Run:

```sh
git commit "Your message here"
git push
```

## Release

Release a new version:

```sh
top=$(git rev-parse --show-toplevel) &&
version=$(gawk 'match($0, /^version = "([^"]*)"/, a) {print a[1]; exit;}' "$top/Cargo.toml") &&
git tag "$version" &&
git push --tags &&
cargo publish
```

Confirm the crate is published:

* <https://crates.io/crates/nhs-number>
