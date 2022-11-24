use super::{color::Color, illuminant::Illuminant, position::AlphaPosition};

#[derive(Debug, Clone, Copy)]
pub struct ColorFormatter {
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
        }
    }
}

impl ColorFormatter {
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
        match self.alpha_position {
            AlphaPosition::End => format!(
                "hsla({}, {}%, {}%, {})",
                hue,
                saturation,
                lightness,
                //convert from [0-255] to [0-1]
                self.pretty_print_percent(
                    self.round_percentage(self.color.alpha as f32 / 255f32) as f32 / 100f32
                )
            ),
            //normal format for non-alpha/ alpha at start
            _ => format!("hsl({}, {}%, {}%)", hue, saturation, lightness),
        }
    }

    /// Format the color as HSV.
    pub fn hsv(&self) -> String {
        let hsv = self.color.to_hsv();
        format!(
            "hsv({}, {}%, {}%)",
            hsv.0,
            self.round_percentage(hsv.1),
            self.round_percentage(hsv.2)
        )
    }

    /// Format the color as CMYK.
    pub fn cmyk(&self) -> String {
        let cmyk = self.color.to_cmyk();
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
        let xyz = self.color.to_xyz();
        format!(
            "XYZ({:.precision$}, {:.precision$}, {:.precision$})",
            xyz.0,
            xyz.1,
            xyz.2,
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
        let cie_lab = self
            .color
            .to_cie_lab(self.illuminant, self.ten_deg_observer);
        format!(
            "CIELAB({:.precision$}, {:.precision$}, {:.precision$})",
            cie_lab.0,
            cie_lab.1,
            cie_lab.2,
            precision = self.precision()
        )
    }

    /// Format the color as HWB.
    pub fn hwb(&self) -> String {
        let hwb = self.color.to_hwb();
        format!(
            "hwb({}, {}%, {}%)",
            hwb.0,
            self.round_percentage(hwb.1),
            self.round_percentage(hwb.2)
        )
    }

    /// Format the color as CIELCh / HCL.
    pub fn hcl(&self) -> String {
        let hcl = self.color.to_hcl(self.illuminant, self.ten_deg_observer);
        format!(
            "lch({:.precision$}, {:.precision$}, {:.precision$})",
            hcl.0,
            hcl.1,
            hcl.2,
            precision = self.precision()
        )
    }

    /// Format the color as LMS.
    pub fn lms(&self) -> String {
        let lms = self.color.to_lms();
        format!(
            "L: {:.precision$}, M: {:.precision$}, S: {:.precision$}",
            lms.0,
            lms.1,
            lms.2,
            precision = self.precision()
        )
    }
    /// Format the color as hunter-lab.
    pub fn hunter_lab(&self) -> String {
        let hunter_lab = self
            .color
            .to_hunter_lab(self.illuminant, self.ten_deg_observer);
        format!(
            "L: {:.precision$}, a: {:.precision$}, b: {:.precision$}",
            hunter_lab.0,
            hunter_lab.1,
            hunter_lab.2,
            precision = self.precision()
        )
    }
}
