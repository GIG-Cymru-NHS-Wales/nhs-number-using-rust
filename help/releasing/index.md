# Releasing

Releasing a new version needs a few steps that we want to automate, yet are currently manual:

## Update the version in file `Cargo.toml`

Edit the file `Cargo.toml`.

Update the version number.

## Verify

Verify everything is correct locally:

```sh
cargo build --release
cargo test
cargo doc
rustdoc-llms
```

## Commit

Run:

```sh
git commit "Your message here"
```

## Tag

Run:

```sh
top=$(git rev-parse --show-toplevel) &&
version=$(gawk 'match($0, /^version = "([^"]*)"/, a) {print a[1]; exit;}' "$top/Cargo.toml") &&
git tag "$version"
```

## Push

Run:

```sh
git push --tags
```

## Publish

Run:

```sh
cargo publish
```

Confirm the crate is published:

* <https://crates.io/crates/nhs-number>
