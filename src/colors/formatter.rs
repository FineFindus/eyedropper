use bytes::{BufMut, BytesMut};
use gtk::prelude::SettingsExt;
use palette::IntoColor;

use crate::config;

use super::{color::Color, illuminant::Illuminant, position::AlphaPosition};

#[derive(Debug, Clone)]
pub struct ColorFormatter {
    settings: gtk::gio::Settings,
    pub color: Color,
    pub precision: usize,
    pub alpha_position: AlphaPosition,
    pub illuminant: Illuminant,
    pub ten_deg_observer: bool,
}

impl Default for ColorFormatter {
    fn default() -> Self {
        let settings = gtk::gio::Settings::new(config::APP_ID);
        Self {
            color: Default::default(),
            precision: settings.uint("precision-digits") as usize,
            alpha_position: AlphaPosition::from(settings.int("alpha-position") as u32),
            illuminant: Illuminant::from(settings.int("cie-illuminants") as u32),
            ten_deg_observer: settings.int("cie-standard-observer") == 1,
            settings,
        }
    }
}

macro_rules! custom_format {
    ($custom_format:expr, $($element:expr),+) => {
        if let Some(mut format) = $custom_format {
            $(
                let pattern = format!("{{{}}}", $element.0);
                format = format.replacen(&pattern, &$element.1.to_string(), 1);)+
            return format;
        }
    };
}

impl ColorFormatter {
    /// Create a new formatter,
    /// which can be used to format colors as human readable strings.
    pub fn new(
        ten_deg_observer: bool,
        illuminant: Illuminant,
        alpha_position: AlphaPosition,
        precision: usize,
        color: Color,
    ) -> Self {
        Self {
            ten_deg_observer,
            illuminant,
            alpha_position,
            precision,
            color,
            ..Default::default()
        }
    }

    /// Create a formatter for the color.
    ///
    /// The formatter will use default values.
    pub fn with_color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    /// Create a formatter with a color and a alpha position.
    pub fn with_alpha_position(color: Color, alpha_position: AlphaPosition) -> Self {
        Self {
            color,
            alpha_position,
            ..Default::default()
        }
    }

    /// Returns the user preference string.
    /// If the use has not specified one, `None` is returned.
    fn custom_format(&self, key: &str) -> Option<String> {
        let value = self.settings.string(key).to_string();
        if value.trim().is_empty() {
            None
        } else {
            Some(value)
        }
    }

    /// Round a percentage value.
    pub fn round_percentage(&self, value: f32) -> f32 {
        (value * 100f32).round()
    }

    /// Get the precision that should be used.
    ///
    /// If the default_precision is set to true, 2 is returned.
    /// Otherwise the precision.
    fn precision(&self) -> usize {
        self.precision
    }

    /// Returns a prettified string of the given value in range [0; 1].
    ///
    /// This function exists to remove trailing zeros from percentages, for example
    /// 0% will be returned as `0` instead of `0.00`
    ///
    ///
    /// If the value is 1, it will be returned as a "1" string.
    /// If it is 0, it will be returned as 0, otherwise it will be returned
    /// as the value with two digits after the comma.
    ///
    /// # Examples
    ///  ```
    /// let value = 0f32;
    /// assert_eq!("0", &pretty_print_percent(value));
    /// ```
    pub fn pretty_print_percent(&self, value: f32) -> String {
        if value >= 1f32 {
            String::from("1")
        } else if value == 0f32 {
            String::from("0")
        } else {
            format!("{:.2}", value)
        }
    }

    /// Format the color as a Hex code.
    ///
    /// Depending on the alpha position,
    /// the alpha is ignored, shown on the start or the end.
    pub fn hex_code(&self) -> String {
        match self.alpha_position {
            AlphaPosition::Start => format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                (self.color.alpha * 255.0) as u8,
                (self.color.red * 255.0) as u8,
                (self.color.green * 255.0) as u8,
                (self.color.blue * 255.0) as u8,
            ),
            AlphaPosition::End => format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                (self.color.red * 255.0) as u8,
                (self.color.green * 255.0) as u8,
                (self.color.blue * 255.0) as u8,
                (self.color.alpha * 255.0) as u8,
            ),
            AlphaPosition::None => format!(
                "#{:02x}{:02x}{:02x}",
                (self.color.red * 255.0) as u8,
                (self.color.green * 255.0) as u8,
                (self.color.blue * 255.0) as u8,
            ),
        }
        .to_ascii_uppercase()
    }

    /// Format the color as RGB.
    ///
    /// Depending on the chosen alpha position,
    /// either rgb or rgba is returned.
    pub fn rgb(&self) -> String {
        custom_format!(
            self.custom_format("custom-format-rgb"),
            ("r", (self.color.red * 255.0).round() as u8),
            ("g", (self.color.green * 255.0).round() as u8),
            ("b", (self.color.blue * 255.0).round() as u8)
        );
        match self.alpha_position {
            //show alpha at the end (rgba)
            AlphaPosition::End => format!(
                "rgba({}, {}, {}, {})",
                (self.color.red * 255.0).round() as u8,
                (self.color.green * 255.0).round() as u8,
                (self.color.blue * 255.0).round() as u8,
                (self.color.alpha * 255.0).round() as u8,
            ),
            // no alpha / there is no argb
            _ => format!(
                "rgb({}, {}, {})",
                (self.color.red * 255.0).round() as u8,
                (self.color.green * 255.0).round() as u8,
                (self.color.blue * 255.0).round() as u8,
            ),
        }
    }

    /// Format the color as HSL.
    pub fn hsl(&self) -> String {
        let hsl: palette::Hsl = self.color.color.into_color();
        //format saturation and lightness to be full percentages
        let saturation = self.round_percentage(hsl.saturation);
        let lightness = self.round_percentage(hsl.lightness);
        custom_format!(
            self.custom_format("custom-format-hsl"),
            ("h", hsl.hue.into_positive_degrees()),
            ("s", saturation),
            ("l", lightness)
        );

        match self.alpha_position {
            AlphaPosition::End => format!(
                "hsla({}, {}%, {}%, {})",
                hsl.hue.into_positive_degrees(),
                saturation,
                lightness,
                //convert from [0-255] to [0-1]
                self.pretty_print_percent(
                    self.round_percentage(self.color.alpha as f32 / 255f32) / 100f32
                )
            ),
            //normal format for non-alpha/ alpha at start
            _ => format!(
                "hsl({}, {}%, {}%)",
                hsl.hue.into_positive_degrees(),
                saturation,
                lightness
            ),
        }
    }

    /// Format the color as HSV.
    pub fn hsv(&self) -> String {
        let hsv: palette::Hsv = self.color.color.into_color();
        custom_format!(
            self.custom_format("custom-format-hsv"),
            ("h", hsv.hue.into_positive_degrees()),
            ("s", self.round_percentage(hsv.saturation)),
            ("v", self.round_percentage(hsv.value * 100.0))
        );

        format!(
            "hsv({}, {}%, {}%)",
            hsv.hue.into_positive_degrees(),
            self.round_percentage(hsv.saturation),
            self.round_percentage(hsv.value)
        )
    }

    /// Format the color as CMYK.
    pub fn cmyk(&self) -> String {
        let cmyk = self.color.to_cmyk();
        custom_format!(
            self.custom_format("custom-format-cmyk"),
            ("c", self.round_percentage(cmyk.0)),
            ("m", self.round_percentage(cmyk.1)),
            ("y", self.round_percentage(cmyk.2)),
            ("k", self.round_percentage(cmyk.3))
        );

        format!(
            "cmyk({}%, {}%, {}%, {}%)",
            self.round_percentage(cmyk.0),
            self.round_percentage(cmyk.1),
            self.round_percentage(cmyk.2),
            self.round_percentage(cmyk.3)
        )
    }

    /// Format the color as XYZ
    pub fn xyz(&self) -> String {
        let xyz: palette::Xyz = self.color.color.into_color();
        custom_format!(
            self.custom_format("custom-format-xyz"),
            ("x", xyz.x * 100.0),
            ("y", xyz.y * 100.0),
            ("z", xyz.z * 100.0)
        );
        format!(
            "XYZ({:.precision$}, {:.precision$}, {:.precision$})",
            xyz.x * 100.0,
            xyz.y * 100.0,
            xyz.z * 100.0,
            precision = self.precision()
        )
    }

    /// Format the color as CIE-Lab.
    pub fn cie_lab(&self) -> String {
        let lab: palette::Lab = self.color.color.into_color();
        custom_format!(
            self.custom_format("custom-format-cie-lab"),
            ("l", lab.l),
            ("a", lab.a),
            ("b", lab.b)
        );
        format!(
            "lab({:.precision$}, {:.precision$}, {:.precision$})",
            lab.l,
            lab.a,
            lab.b,
            precision = self.precision()
        )
    }

    /// Format the color as HWB.
    pub fn hwb(&self) -> String {
        let hwb: palette::Hwb = self.color.color.into_color();
        custom_format!(
            self.custom_format("custom-format-hwb"),
            ("h", hwb.hue.into_positive_degrees()),
            ("w", self.round_percentage(hwb.whiteness)),
            ("b", self.round_percentage(hwb.blackness))
        );
        format!(
            "hwb({}, {}%, {}%)",
            hwb.hue.into_positive_degrees(),
            self.round_percentage(hwb.whiteness),
            self.round_percentage(hwb.blackness)
        )
    }

    /// Format the color as CIELCh / HCL.
    pub fn hcl(&self) -> String {
        let lch: palette::Lch = self.color.color.into_color();
        custom_format!(
            self.custom_format("custom-format-hcl"),
            ("h", lch.hue.into_positive_degrees()),
            ("c", lch.chroma),
            ("l", lch.l)
        );
        format!(
            "lch({:.precision$}, {:.precision$}, {:.precision$})",
            lch.l,
            lch.chroma,
            lch.hue.into_positive_degrees(),
            precision = self.precision()
        )
    }

    /// Format the color as LMS.
    pub fn lms(&self) -> String {
        let (l, m, s) = self.color.to_lms();
        custom_format!(
            self.custom_format("custom-format-lms"),
            ("l", l),
            ("m", m),
            ("s", s)
        );
        format!(
            "L: {:.precision$}, M: {:.precision$}, S: {:.precision$}",
            l,
            m,
            s,
            precision = self.precision()
        )
    }
    /// Format the color as hunter-lab.
    pub fn hunter_lab(&self) -> String {
        let (l, a, b) = self
            .color
            .to_hunter_lab(self.illuminant, self.ten_deg_observer);
        custom_format!(
            self.custom_format("custom-format-hunter-lab"),
            ("l", l),
            ("a", a),
            ("b", b)
        );
        format!(
            "L: {:.precision$}, a: {:.precision$}, b: {:.precision$}",
            l,
            a,
            b,
            precision = self.precision()
        )
    }

    /// Format the color as Oklab
    pub fn oklab(&self) -> String {
        let oklab: palette::Oklab = self.color.color.into_color();

        custom_format!(
            self.custom_format("custom-format-oklab"),
            ("l", self.round_percentage(oklab.l)),
            ("a", oklab.a),
            ("b", oklab.b)
        );

        match self.alpha_position {
            AlphaPosition::End => format!(
                "oklab({}% {:.precision$} {:.precision$} / {})",
                self.round_percentage(oklab.l),
                oklab.a,
                oklab.b,
                //convert from [0-255] to [0-1]
                self.pretty_print_percent(
                    self.round_percentage(self.color.alpha as f32 / 255f32) / 100f32
                ),
                precision = self.precision(),
            ),
            //normal format for non-alpha/ alpha at start
            _ => format!(
                "oklab({}% {:.precision$} {:.precision$})",
                self.round_percentage(oklab.l),
                oklab.a,
                oklab.b,
                precision = self.precision()
            ),
        }
    }

    /// Format the color as Oklch
    pub fn oklch(&self) -> String {
        let oklch: palette::Oklch = self.color.color.into_color();

        custom_format!(
            self.custom_format("custom-format-oklch"),
            ("lightness", self.round_percentage(oklch.l)),
            ("chroma", oklch.chroma),
            ("hue", oklch.hue.into_positive_degrees())
        );

        match self.alpha_position {
            AlphaPosition::End => format!(
                "oklch({}% {:.precision$} {:.precision$} / {})",
                self.round_percentage(oklch.l),
                oklch.chroma,
                oklch.hue.into_positive_degrees(),
                //convert from [0-255] to [0-1]
                self.pretty_print_percent(
                    self.round_percentage(self.color.alpha as f32 / 255f32) / 100f32
                ),
                precision = self.precision(),
            ),
            //normal format for non-alpha/ alpha at start
            _ => format!(
                "oklch({}% {:.precision$} {:.precision$})",
                self.round_percentage(oklch.l),
                oklch.chroma,
                oklch.hue.into_positive_degrees(),
                precision = self.precision()
            ),
        }
    }

    /// Format the colors as a GIMP palette file.
    ///
    /// The name will be the name of the palette, each color will be
    /// named untitled.
    pub fn gpl_file(name: &str, colors: &[Color]) -> String {
        let mut content = format!(
            "GIMP Palette\n\
            Name: {name}\n\
            Columns: 0\n\
            # Palette file generated by Eyedropper\n",
        );

        colors.iter().for_each(|color| {
            content.push_str(&format!(
                //don't add a name
                "{:>3} {:>3} {:>3}	Untitled\n",
                color.red, color.green, color.blue,
            ))
        });

        content
    }

    /// Format the colors as a [PAINT.net palette file](https://www.getpaint.net/doc/latest/WorkingWithPalettes.html).
    ///
    /// The name will be the name of the palette.
    pub fn paint_dot_net_file(name: &str, colors: &[Color]) -> String {
        let mut content = format!(
            ";paint.net Palette File\n\
            ;Palette Name: {}\n\
            ;Description: Palette file generated by Eyedropper\n\
            ;Colors: {}\n",
            name,
            colors.len()
        );

        colors
            .iter()
            .map(|&color| Self::with_alpha_position(color, AlphaPosition::Start))
            .for_each(|formatter| {
                content.push_str(&formatter.hex_code());
                content.push('\n');
            });

        content
    }

    /// Format the colors as a .pal file, used by e.g. Corel Painter.
    ///
    /// The name will be the name of the palette, each color will be
    /// named untitled.
    ///
    /// While some apps accept PAL formats with RGBA, most common is a version without alpha.
    /// The saved file will not contain alpha values, there not generated in the pallettes,
    /// so there would be no value in writing them out
    pub fn pal_file(colors: &[Color]) -> String {
        //save magic letters, version number and number of colors
        let mut content = format!(
            "JASC-PAL\n\
            0100\n\
            {}\n",
            colors.len()
        );

        colors.iter().for_each(|&color| {
            content.push_str(&format!("{} {} {}\n", color.red, color.green, color.blue));
        });

        content
    }

    /// Format the colors as a .hex file
    ///
    /// This format contains only the raw hex strings, without alpha values and no leading # symbols.
    pub fn hex_file(colors: &[Color]) -> String {
        let mut content = String::with_capacity(8 * colors.len());

        colors
            .iter()
            .map(|&color| Self::with_color(color))
            .for_each(|formatter| {
                let mut hex_string = formatter.hex_code();
                hex_string.remove(0);
                content.push_str(&hex_string);
                content.push('\n');
            });

        content
    }

    /// Format the palette file for export usage in LibreOffice.
    ///
    /// Colors will be named as the hex color, as each color needs a different name.
    /// Created according to the documentation at <https://wiki.documentfoundation.org/Videos/Create_color_palette>
    pub fn soc_file(colors: &[Color]) -> String {
        let mut content = String::new();
        content.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        content.push('\n');
        content.push_str(r#"<ooo:color-table xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:svg="http://www.w3.org/2000/svg" xmlns:ooo="http://openoffice.org/2004/office">"#);
        content.push('\n');

        content.push_str(
            &colors
                .iter()
                .map(|&color| Self::with_color(color))
                .map(|formatter| {
                    format!(
                        "<draw:color draw:name=\"{}\" draw:color=\"{}\"/>\n",
                        formatter.hex_code(),
                        formatter.hex_code()
                    )
                })
                .collect::<String>(),
        );

        content.push_str("</ooo:color-table>");
        content
    }

    /// Format the colors as a `.ase` file.
    ///
    /// `.ase` files are used by Adobe products and have no public spec, this implementation follows a
    /// [blog post by car.camera](https://carl.camera/default.aspx?id=109) and
    /// http://www.selapa.net/swatches/colors/fileformats.php#adobe_ase.
    pub fn ase_file(colors: &[Color]) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(12 + colors.len() * 42);

        //magic header letters
        buf.put(&b"ASEF"[..]);

        //version number 0x00010000
        buf.put_u32(0x00010000);

        //number of following 'ASECHUNK's
        buf.put_u32(colors.len() as u32);

        for &color in colors {
            let hex = Self::with_alpha_position(color, AlphaPosition::None).hex_code();

            //start of color entry
            buf.put_u16(0x0001);

            //block length
            //the length is calculated the following way:
            // 2 [name length indicator] + hex.len() as u32 * 2 [name as u16 bytes] + 2 [terminator] +
            // 4 [RGB indicator] + (3 * 4) [values] + 2 [type] = 26
            buf.put_u32(36);

            //name length, +1 for null terminator
            buf.put_u16(hex.len() as u16 + 1);
            hex.encode_utf16().for_each(|byte| buf.put_u16(byte));
            //null terminator
            buf.put_u16(0);

            //color
            buf.put(&b"RGB"[..]);
            buf.put_u8(0x20);
            buf.put_f32(color.red as f32 / 255f32);
            buf.put_f32(color.green as f32 / 255f32);
            buf.put_f32(color.blue as f32 / 255f32);

            //type (0 = Global, 1 = Spot, 2 = Normal)
            buf.put_u16(0);
        }

        buf.freeze().to_vec()
    }
}
