# Publish checklist

Before publishing version `X.Y.Z`:

- [ ] Bump version in `enum-helper/Cargo.toml`
- [ ] Bump version in `enum-helper-derive/Cargo.toml`
- [ ] Bump the `enum-helper-derive` dependency version in `enum-helper/Cargo.toml`
- [ ] `cargo build` to update `Cargo.lock`
- [ ] Add `## [X.Y.Z] - YYYY-MM-DD` to `CHANGELOG.md`
- [ ] Check `README.md` version example matches
- [ ] `just ci`

Publish (derive first, then the main crate):

- [ ] `cargo publish -p enum-helper-derive`
- [ ] `cargo publish -p enum-helper`
- [ ] Tag `vX.Y.Z` and push