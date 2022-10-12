# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- Options to show names of color (from w3c basic, extended and xkcd)
- Palettes are now shown in the palette dialog, accessible by clicking on the large color button
- The App now uses the AdwAboutWindow
- HSL does now support alpha values
- HWB and CIELCh are now supported
- The following translations have been added
    - Spanish by [IngrownMink4](https://github.com/IngrownMink4)  
    - Basque by [IngrownMink4](https://github.com/IngrownMink4)  
    - Turkish by [libreajans](https://github.com/libreajans)  
    - Dutch by [flipflop97](https://github.com/flipflop97)  
    - Italian by [phaerrax](https://github.com/phaerrax)
- Code refactoring 

### Removed
- Removed explicit dependency on the `libc` crate, as it is no longer needed with the new palette dialog

## [0.3.1] - 2022-09-23

### Added

- Translation for the "Copied" message
- A symbolic icon

### Changed

- Fixed broken Alphaposition setting

## [0.3.0] - 2022-09-21

### Added

- Palettes, consisting of darker shades and lighter tints, are now generated from the currently picked color as well as the previous 3
- The app has been translated into French by [rene-coty](https://github.com/rene-coty)
- A German translation has been added
- Change the order of the color formats by dragging and dropping them in the settings
- Adjusted some strings to be easier to understand

## [0.2.0] - 2022-09-08

### Added

- Issue and feature-request templates
- Change the color using hex values in the hex field
- Previous colors are now visible in the history list
- Disable unwanted color formats
- Colors can now be formatted as XYZ or CIELAB

## [0.1.0] - 2022-08-28

### Added

- Basic UI
- Pick a color using the color picker button
- Copy the color in different color models
- Change the position of the alpha values in hex values
- Added License
