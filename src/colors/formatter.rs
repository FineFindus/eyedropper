use gtk::prelude::SettingsExt;

use crate::config;

use super::{color::Color, illuminant::Illuminant, position::AlphaPosition};

#[derive(Debug, Clone)]
pub struct ColorFormatter {
    settings: gtk::gio::Settings,
    pub color: Color,
    pub default_precision: bool,
    pub precision: usize,
    pub alpha_position: AlphaPosition,
    pub illuminant: Illuminant,
    pub ten_deg_observer: bool,
}

impl Default for ColorFormatter {
    fn default() -> Self {
        Self {
            color: Default::default(),
            default_precision: true,
            precision: 2,
            alpha_position: Default::default(),
            illuminant: Default::default(),
            ten_deg_observer: Default::default(),
            settings: gtk::gio::Settings::new(config::APP_ID),
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
        default_precision: bool,
        precision: usize,
        color: Color,
    ) -> Self {
        Self {
            ten_deg_observer,
            illuminant,
            alpha_position,
            default_precision,
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
        if self.default_precision {
            2
        } else {
            self.precision
        }
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
                self.color.alpha, self.color.red, self.color.green, self.color.blue
            ),
            AlphaPosition::End => format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                self.color.red, self.color.green, self.color.blue, self.color.alpha
            ),
            AlphaPosition::None => format!(
                "#{:02x}{:02x}{:02x}",
                self.color.red, self.color.green, self.color.blue
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
            ("r", self.color.red),
            ("g", self.color.green),
            ("b", self.color.blue)
        );
        match self.alpha_position {
            //show alpha at the end (rgba)
            AlphaPosition::End => format!(
                "rgba({}, {}, {}, {})",
                self.color.red, self.color.green, self.color.blue, self.color.alpha
            ),
            // no alpha / there is no argb
            _ => format!(
                "rgb({}, {}, {})",
                self.color.red, self.color.green, self.color.blue
            ),
        }
    }

    /// Format the color as HSL.
    pub fn hsl(&self) -> String {
        let (hue, saturation, lightness) = self.color.to_hsl();
        //format saturation and lightness to be full percentages
        let saturation = self.round_percentage(saturation);
        let lightness = self.round_percentage(lightness);
        custom_format!(
            self.custom_format("custom-format-hsl"),
            ("h", hue),
            ("s", saturation),
            ("l", lightness)
        );

        match self.alpha_position {
            AlphaPosition::End => format!(
                "hsla({}, {}%, {}%, {})",
                hue,
                saturation,
                lightness,
                //convert from [0-255] to [0-1]
                self.pretty_print_percent(
                    self.round_percentage(self.color.alpha as f32 / 255f32) / 100f32
                )
            ),
            //normal format for non-alpha/ alpha at start
            _ => format!("hsl({}, {}%, {}%)", hue, saturation, lightness),
        }
    }

    /// Format the color as HSV.
    pub fn hsv(&self) -> String {
        let (h, s, v) = self.color.to_hsv();
        custom_format!(
            self.custom_format("custom-format-hsv"),
            ("h", h),
            ("s", self.round_percentage(s)),
            ("v", self.round_percentage(v))
        );

        format!(
            "hsv({}, {}%, {}%)",
            h,
            self.round_percentage(s),
            self.round_percentage(v)
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
        let (x, y, z) = self.color.to_xyz();
        custom_format!(
            self.custom_format("custom-format-xyz"),
            ("x", x),
            ("y", y),
            ("z", z)
        );
        format!(
            "XYZ({:.precision$}, {:.precision$}, {:.precision$})",
            x,
            y,
            z,
            //this is the only format that has 3 digit precision by default, override the default precision
            precision = if self.default_precision {
                3
            } else {
                self.precision()
            }
        )
    }

    /// Format the color as CIE-Lab.
    pub fn cie_lab(&self) -> String {
        let (l, a, b) = self
            .color
            .to_cie_lab(self.illuminant, self.ten_deg_observer);
        custom_format!(
            self.custom_format("custom-format-cie-lab"),
            ("l", l),
            ("a", a),
            ("b", b)
        );
        format!(
            "CIELAB({:.precision$}, {:.precision$}, {:.precision$})",
            l,
            a,
            b,
            precision = self.precision()
        )
    }

    /// Format the color as HWB.
    pub fn hwb(&self) -> String {
        let (h, w, b) = self.color.to_hwb();
        custom_format!(
            self.custom_format("custom-format-hwb"),
            ("h", h),
            ("w", self.round_percentage(w)),
            ("b", self.round_percentage(b))
        );
        format!(
            "hwb({}, {}%, {}%)",
            h,
            self.round_percentage(w),
            self.round_percentage(b)
        )
    }

    /// Format the color as CIELCh / HCL.
    pub fn hcl(&self) -> String {
        let (h, c, l) = self.color.to_hcl(self.illuminant, self.ten_deg_observer);
        custom_format!(
            self.custom_format("custom-format-hcl"),
            ("h", h),
            ("c", c),
            ("l", l)
        );
        format!(
            "lch({:.precision$}, {:.precision$}, {:.precision$})",
            h,
            c,
            l,
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

    /// Format the colors as a GIMP palette file.
    ///
    /// The name will be the name of the palette, each color will be
    /// named untitled.
    pub fn gpl_file(name: &str, colors: Vec<Color>) -> String {
        let mut content = format!(
            "GIMP Palette\n\
            Name: {name}\n\
            Columns: 0\n\
            # Palette file generated by Eyedropper\n",
        );

        for color in colors {
            content.push_str(&format!(
                //don't add a name
                "{:>3} {:>3} {:>3}	Untitled\n",
                color.red, color.green, color.blue,
            ));
        }

        content
    }

    /// Format the colors as a PAINT.net file.
    ///
    /// The name will be the name of the palette.
    pub fn paint_dot_net_file(name: &str, colors: Vec<Color>) -> String {
        let mut content = format!(
            ";paint.net Palette File\n\
            ;Palette Name: {}\n\
            ;Description: Palette file generated by Eyedropper\n\
            ;Colors: {}\n",
            name,
            colors.len()
        );

        for color in colors {
            let formatter = Self::with_alpha_position(color, AlphaPosition::Start);
            content.push_str(&formatter.hex_code());
            content.push('\n');
        }

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
    pub fn pal_file(colors: Vec<Color>) -> String {
        //save magic letters, version number and number of colors
        let mut content = format!(
            "JASC-PAL\n\
            0100\n\
            {}\n",
            colors.len()
        );

        for color in colors {
            content.push_str(&format!("{} {} {}\n", color.red, color.green, color.blue));
        }

        content
    }

    /// Format the colors as a .hex file
    ///
    /// This format contains only the raw hex strings, without alpha values and no leading # symbols.
    pub fn hex_file(colors: Vec<Color>) -> String {
        let mut content = String::with_capacity(8 * colors.len());

        for color in colors {
            let formatter = Self::with_alpha_position(color, AlphaPosition::None);
            let mut hex_string = formatter.hex_code();
            hex_string.remove(0);
            content.push_str(&hex_string);
            content.push('\n');
        }
        content
    }
}
