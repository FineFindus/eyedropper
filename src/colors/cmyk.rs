use palette::{convert::FromColorUnclamped, encoding, rgb::Rgb, Clamp, Srgb, WithAlpha};

/// CMYK with a alpha component.
///
/// Based on <https://www.easyrgb.com/en/math.php>
#[derive(Debug, FromColorUnclamped, WithAlpha)]
#[palette(skip_derives(Rgb), rgb_standard = "encoding::Srgb")]
pub struct Cmyka {
    /// The amount of cyan ink, where 0.0 is no cyan ink and 1.0 is the maximum amount.
    pub cyan: f32,
    /// The amount of magenta ink, where 0.0 is no magenta ink and 1.0 is the maximum amount.
    pub magenta: f32,
    /// The amount of yellow ink, where 0.0 is no yellow ink and 1.0 is the maximum amount.
    pub yellow: f32,
    /// The amount of black (key) ink, where 0.0 is no black ink and 1.0 is the maximum amount.
    pub k: f32,
    /// The opacity of the color, where 0.0 is fully transparent and 1.0 is fully opaque.
    #[palette(alpha)]
    pub alpha: f32,
}

impl Cmyka {
    /// Create a CMYK color with transparency.
    pub fn new(cyan: f32, magenta: f32, yellow: f32, k: f32, alpha: f32) -> Self {
        Self {
            cyan,
            magenta,
            yellow,
            k,
            alpha,
        }
    }
}

impl FromColorUnclamped<Cmyka> for Cmyka {
    fn from_color_unclamped(color: Cmyka) -> Cmyka {
        color
    }
}

impl<S> FromColorUnclamped<Rgb<S, f32>> for Cmyka
where
    Srgb: FromColorUnclamped<Rgb<S, f32>>,
{
    fn from_color_unclamped(color: Rgb<S, f32>) -> Cmyka {
        let srgb = Srgb::from_color_unclamped(color);
        let mut c = 1f32 - srgb.red;
        let mut m = 1f32 - srgb.green;
        let mut y = 1f32 - srgb.blue;

        let mut k = 1f32;
        if c < k {
            k = c;
        }
        if m < k {
            k = m;
        }
        if y < k {
            k = y;
        }

        if k == 1f32 {
            //only black
            c = 0f32;
            m = 0f32;
            y = 0f32;
        } else {
            c = (c - k) / (1f32 - k);
            m = (m - k) / (1f32 - k);
            y = (y - k) / (1f32 - k);
        }

        Cmyka {
            cyan: c,
            magenta: m,
            yellow: y,
            k,
            alpha: 1.0,
        }
    }
}

impl<S> FromColorUnclamped<Cmyka> for Rgb<S, f32>
where
    Rgb<S, f32>: FromColorUnclamped<Srgb>,
{
    fn from_color_unclamped(color: Cmyka) -> Self {
        let srgb = Srgb::new(
            (1f32 - color.cyan) * (1f32 - color.k),
            (1f32 - color.magenta) * (1f32 - color.k),
            (1f32 - color.yellow) * (1f32 - color.k),
        );
        Self::from_color_unclamped(srgb)
    }
}

impl Clamp for Cmyka {
    fn clamp(self) -> Self {
        Cmyka {
            cyan: self.cyan.clamp(0.0, 1.0),
            magenta: self.magenta.clamp(0.0, 1.0),
            yellow: self.yellow.clamp(0.0, 1.0),
            k: self.k.clamp(0.0, 1.0),
            alpha: self.alpha.clamp(0.0, 1.0),
        }
    }
}
