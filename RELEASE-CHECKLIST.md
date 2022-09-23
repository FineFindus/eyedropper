# Release Checklist

These steps should be performed when making a new release. Do not commit marked checks in this file.

## Pre-release Test Checklist

### Before Committing

- [ ] Local and Remote branches are synced
- [ ] All the tests are passing
- [ ] Continuous Integration is passing
- [ ] `cargo clippy` finds no errors
- [ ] Flatpak can be build from the tarball
- [ ] README is up-to-date
- [ ] Build version matches version in Cargo.toml
- [ ] Example images still represents the project accurately
- [ ] Documentation has been updated to reflect the changes
- [ ] [Translation Template](po/Eyedropper.pot) is updated
- [ ] If possible: App was translated

#### Changelog

- [ ] All changes were documented in the Changelog
- [ ] All changes were documented in the Metainfo-file
- [ ] Added the correct semantic version in the Changelog
- [ ] Changed the changes from Unreleased to the new version in the Changelog

#### Version

The following Version numbers have been updates

- [ ] [CHANGELOG.md](CHANGELOG.md)
- [ ] [Cargo.toml](Cargo.toml)
- [ ] [config.rs](src/config.rs)
- [ ] [meson.build](meson.build)
- [ ] [metainfo.xml](data/com.github.finefindus.eyedropper.metainfo.xml.in.in)

### After Committing

- [ ] Copied the changes to a new release
- [ ] Build artifacts have been attached to the release through continuous delivery
- [ ] Tarball for Flathub has been uploaded
- [ ] Sha256sum of the tarball been uploaded

## Post-release Test Checklist

- [ ] Installation instructions work using the release artifact
- [ ] Updated Flathub Manifest
