# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

### Added
- Show a toast with an Undo option when clearing the history
- Use Ctrl-Q to close the window
- Use a default minimum window size, content should no longer unnecessarily cut off
- New icon by @bertob
- Export palettes as Adobe Swatch Exchange (ASE), hex, PAL, Paint.NET  (txt) files
- Minor changes to the store metadata

### Changed
- Hunter-Lab now displays 0 instead of NaN
- Improved tooltips for copying colors
- Palette window now uses a button with a more explicit text instead of an icon
- Reworded project description
- Updated all screenshots

### Fixed
- Separator is now hidden when the history list is hidden
- Improved editing experience when change the Hex color/name

## [0.5.1] - 2023-01-29

### Added 
- Russian translation by @vorons
- Use higher optimization options when compiling a release 
- Return the exit code when quitting

### Changed
- History no longer contains duplicated colors

### Fixed
- Wrong default `rgb` format
- LMS and Hunter Lab sharing their preference setting

## [0.5.0] - 2023-01-03

### Added
- Export the generated palettes from the palette dialog as a GIMP palette file
- A `.doap` file to describe the project 
- Show a placeholder page when no color is picked
- LMS color space can now be shown
- Convert to Hunter-Lab color space
- Customize color formats

### Changed
- Fixed a bug where the illuminants were not fully shown
- The names of the GNOME color palette are now available

## [0.4.0] - 2022-10-20

### Added
- Options to show names of color (from w3c basic, extended and xkcd)
- Change the color by typing a name in the name field
- Palettes are now shown in the palette dialog, accessible by clicking on the large color button
- The App now uses the AdwAboutWindow
- HSL does now support alpha values
- HWB and CIELCh are now supported
- CIE standard observer for CIELab and CIELCh
- Change CIE illuminants
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
