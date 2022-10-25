/// Different Illuminants for CIELab calculation.
///
/// Values from <https://www.easyrgb.com/en/math.php>
#[derive(Debug, Clone, Copy, Default)]
pub enum Illuminant {
    ///Incandescent/tungsten
    A,
    ///Old direct sunlight at noon
    B,
    ///Old daylight
    C,
    ///ICC profile PCS
    D50,
    ///Mid-morning daylight
    D55,
    ///Daylight, sRGB, Adobe-RGB
    #[default]
    D65,
    ///North sky daylight
    D75,
    ///Equal energy
    E,
    ///Daylight Fluorescent
    F1,
    ///Cool fluorescent
    F2,
    ///White Fluorescent
    F3,
    ///Warm White Fluorescent
    F4,
    ///Daylight Fluorescent
    F5,
    ///Lite White Fluorescent
    F6,
    ///Daylight fluorescent, D65 simulator
    F7,
    ///Sylvania F40, D50 simulator
    F8,
    ///Cool White Fluorescent
    F9,
    ///Ultralume 50, Philips TL85
    F10,
    ///Ultralume 40, Philips TL84
    F11,
    ///Ultralume 30, Philips TL83
    F12,
}

impl Illuminant {
    /// 2° CIE-Illuminants
    ///
    /// Returns the values for the illuminants with a 2 degree
    /// field-of-view.
    ///
    /// These are older, but much more commonly used then the 10 degree illuminants.
    pub fn two_degrees(self) -> (f32, f32, f32) {
        match self {
            Illuminant::A => (109.850, 100.0, 35.585),
            Illuminant::B => (99.0927, 100.0, 85.313),
            Illuminant::C => (98.074, 100.0, 118.232),
            Illuminant::D50 => (96.422, 100.0, 82.521),
            Illuminant::D55 => (95.682, 100.0, 92.149),
            Illuminant::D65 => (95.047, 100.0, 108.883),
            Illuminant::D75 => (94.972, 100.0, 122.638),
            Illuminant::E => (100.0, 100.0, 100.0),
            Illuminant::F1 => (92.834, 100.0, 103.665),
            Illuminant::F2 => (99.187, 100.0, 67.395),
            Illuminant::F3 => (103.754, 100.0, 49.861),
            Illuminant::F4 => (109.147, 100.0, 38.813),
            Illuminant::F5 => (90.872, 100.0, 98.723),
            Illuminant::F6 => (97.309, 100.0, 60.191),
            Illuminant::F7 => (95.044, 100.0, 108.755),
            Illuminant::F8 => (96.413, 100.0, 82.333),
            Illuminant::F9 => (100.365, 100.0, 67.868),
            Illuminant::F10 => (96.174, 100.0, 81.712),
            Illuminant::F11 => (100.966, 100.0, 64.370),
            Illuminant::F12 => (108.046, 100.0, 39.228),
        }
    }

    /// 10° CIE-Illuminants.
    ///
    /// Returns the values for the illuminants with a 10 degree
    /// field-of-view.
    pub fn ten_degrees(self) -> (f32, f32, f32) {
        match self {
            Illuminant::A => (111.144, 100.0, 35.200),
            Illuminant::B => (99.178, 100.0, 84.3493),
            Illuminant::C => (97.285, 100.0, 116.145),
            Illuminant::D50 => (96.720, 100.0, 81.427),
            Illuminant::D55 => (95.799, 100.0, 90.926),
            Illuminant::D65 => (94.811, 100.0, 107.304),
            Illuminant::D75 => (94.416, 100.0, 120.641),
            Illuminant::E => (100.0, 100.0, 100.0),
            Illuminant::F1 => (94.791, 100.0, 103.191),
            Illuminant::F2 => (103.280, 100.0, 69.026),
            Illuminant::F3 => (108.968, 100.0, 51.965),
            Illuminant::F4 => (114.961, 100.0, 40.963),
            Illuminant::F5 => (93.369, 100.0, 98.636),
            Illuminant::F6 => (102.148, 100.0, 62.074),
            Illuminant::F7 => (95.792, 100.0, 107.687),
            Illuminant::F8 => (97.115, 100.0, 81.135),
            Illuminant::F9 => (102.116, 100.0, 67.826),
            Illuminant::F10 => (99.001, 100.0, 83.134),
            Illuminant::F11 => (103.866, 100.0, 65.627),
            Illuminant::F12 => (111.428, 100.0, 40.353),
        }
    }
}

impl From<u32> for Illuminant {
    fn from(value: u32) -> Self {
        match value {
            0 => Illuminant::A,
            1 => Illuminant::B,
            2 => Illuminant::C,
            3 => Illuminant::D50,
            4 => Illuminant::D55,
            5 => Illuminant::D65,
            6 => Illuminant::D75,
            7 => Illuminant::E,
            8 => Illuminant::F1,
            9 => Illuminant::F2,
            10 => Illuminant::F3,
            11 => Illuminant::F4,
            12 => Illuminant::F5,
            13 => Illuminant::F6,
            14 => Illuminant::F7,
            15 => Illuminant::F8,
            16 => Illuminant::F9,
            17 => Illuminant::F10,
            18 => Illuminant::F11,
            19 => Illuminant::F12,
            //default to D65
            _ => Illuminant::D65,
        }
    }
}
