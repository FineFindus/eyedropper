use std::marker::PhantomData;

use palette::{
    Clamp, WithAlpha, Xyz,
    convert::FromColorUnclamped,
    white_point::{D65, WhitePoint},
};

/// HunterLab with an alpha component.
///
/// Based on <https://www.easyrgb.com/en/math.php>
#[derive(Debug, FromColorUnclamped, WithAlpha)]
#[palette(skip_derives(Xyz))]
pub struct HunterLab<Wp = D65> {
    pub l: f32,
    /// The amount of magenta ink, where 0.0 is no magenta ink and 1.0 is the maximum amount.
    pub a: f32,
    /// The amount of yellow ink, where 0.0 is no yellow ink and 1.0 is the maximum amount.
    pub b: f32,
    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp> HunterLab<Wp> {
    /// Create a HunterLab color
    pub const fn new(l: f32, a: f32, b: f32) -> Self {
        Self {
            l,
            a,
            b,
            white_point: PhantomData,
        }
    }
}

impl FromColorUnclamped<HunterLab> for HunterLab {
    fn from_color_unclamped(color: HunterLab) -> HunterLab {
        color
    }
}

impl<Wp> FromColorUnclamped<Xyz<Wp, f32>> for HunterLab<Wp>
where
    Wp: WhitePoint<f32>,
{
    fn from_color_unclamped(color: Xyz<Wp, f32>) -> Self {
        let wp = Wp::get_xyz();

        let ka = (175.0 / 198.04) * (wp.x + wp.y);
        let kb = (70.0 / 218.11) * (wp.y + wp.z);

        let l = 100.0 * f32::sqrt(color.y / wp.y);
        let a = ka * (((color.x / wp.x) - (color.y / wp.y)) / f32::sqrt(color.y / wp.y));
        let b = kb * (((color.y / wp.y) - (color.z / wp.z)) / f32::sqrt(color.y / wp.y));

        Self {
            l,
            a: if a.is_nan() { 0.0 } else { a },
            b: if b.is_nan() { 0.0 } else { b },
            white_point: PhantomData::<Wp>,
        }
    }
}

impl<Wp> FromColorUnclamped<HunterLab<Wp>> for Xyz<Wp, f32>
where
    Wp: WhitePoint<f32>,
{
    fn from_color_unclamped(color: HunterLab<Wp>) -> Self {
        let wp = Wp::get_xyz();
        let ka = (175.0 / 198.04) * (wp.y + wp.x);
        let kb = (70.0 / 218.11) * (wp.y + wp.z);

        let y = ((color.l / wp.y).powi(2)) * 100.0;
        let x = (color.a / ka * (y / wp.y).sqrt() + (y / wp.y)) * wp.x;
        let z = -(color.b / kb * (y / wp.y).sqrt() - (y / wp.y)) * wp.z;

        palette::Xyz::new(x, y, z)
    }
}

impl Clamp for HunterLab {
    fn clamp(self) -> Self {
        HunterLab {
            l: self.l.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            white_point: self.white_point,
        }
    }
}
