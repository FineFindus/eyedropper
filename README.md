![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/FineFindus/eyedropper/actions/workflows/ci.yml/badge.svg)](https://github.com/FineFindus/eyedropper/actions/workflows/ci.yml)

![Eyedropper](data/icons/com.github.finefindus.eyedropper.svg)

# Eyedropper

An easy-to-use color picker and formatter.

![Dark UI](data/resources/screenshots/main_window_ui_dark.png)

<details>
  <summary>More screenshots</summary>
  
![Light UI](data/resources/screenshots/main_window_ui_light.png)

![Customize the shown formats](data/resources/screenshots/customized_formats_dark1.png)

![Customize the shown formats](data/resources/screenshots/customized_formats_dark2.png)

</details>

## Features

- Pick a Color
- Edit a color using hex values
- Parse RGBA/ARGB Colors
- View colors in RGB, HSV, HSL, CMYK, XYZ and CIE-Lab formats

### Future Features & Roadmap

- Generate a palette of similar colors
- Add more color formats

## Installation

### Official

Install from [Flathub](https://flathub.org/apps/details/com.github.finefindus.eyedropper) or download the latest release from the [release tab](https://github.com/FineFindus/eyedropper/releases).

### Inofficial

> :warning: **These installation methods are not recommended**: Installing them may result in bugs or unexpected behavior.

#### Nightly Flatpak

Download the latest artifact from the [CI](https://github.com/FineFindus/eyedropper/actions/workflows/ci.yml).

#### [AUR](https://aur.archlinux.org/packages/eyedropper)

```sh
yay -S eyedropper
```

## Building

See this [general guide](https://wiki.gnome.org/Newcomers/BuildProject) for building the project using GNOME Builder.

Alternatively use this [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=bilelmoussaoui.flatpak-vscode#:~:text=VSCode%20%2B%20Flatpak%20Integration,run%2C%20and%20export%20a%20bundle) for working inside VS Code.

### Building manually

Alternatively, it is possible to build the project manually using `flatpak-builder`.
First install the required sdks:

```sh
flatpak install org.gnome.Sdk//41 org.freedesktop.Sdk.Extension.rust-stable//21.08 org.gnome.Platform//41
```

Then build it using:

```sh
flatpak-builder --user flatpak_app build-aux/com.github.finefindus.eyedropper.Devel.json
```

To run it:

```sh
flatpak-builder --run flatpak_app build-aux/com.github.finefindus.eyedropper.Devel.json eyedropper
```

## Credits

A huge thanks to these projects who served either as an inspiration or as code examples on how to use gtk-rs.

- [GTK Rust Template](https://gitlab.gnome.org/World/Rust/gtk-rust-template)
- [Contrast](https://gitlab.gnome.org/World/design/contrast)
- [Microsoft Color Picker Utility](https://docs.microsoft.com/en-us/windows/powertoys/color-picker) - Inspirations on the design
