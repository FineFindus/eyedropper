/// Different Observers for CIELab calculation.
///
/// Values from <https://www.easyrgb.com/en/math.php>
#[derive(Debug, Clone, Copy, Default)]
pub enum Observer {
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

impl Observer {
    /// Get values
    pub fn two_degrees(self) -> (f32, f32, f32) {
        match self {
            Observer::A => (109.850, 100.0, 35.585),
            Observer::B => (99.0927, 100.0, 85.313),
            Observer::C => (98.074, 100.0, 118.232),
            Observer::D50 => (96.422, 100.0, 82.521),
            Observer::D55 => (95.682, 100.0, 92.149),
            Observer::D65 => (95.047, 100.0, 108.883),
            Observer::D75 => (94.972, 100.0, 122.638),
            Observer::E => (100.0, 100.0, 100.0),
            Observer::F1 => (92.834, 100.0, 103.665),
            Observer::F2 => (99.187, 100.0, 67.395),
            Observer::F3 => (103.754, 100.0, 49.861),
            Observer::F4 => (109.147, 100.0, 38.813),
            Observer::F5 => (90.872, 100.0, 98.723),
            Observer::F6 => (97.309, 100.0, 60.191),
            Observer::F7 => (95.044, 100.0, 108.755),
            Observer::F8 => (96.413, 100.0, 82.333),
            Observer::F9 => (100.365, 100.0, 67.868),
            Observer::F10 => (96.174, 100.0, 81.712),
            Observer::F11 => (100.966, 100.0, 64.370),
            Observer::F12 => (108.046, 100.0, 39.228),
        }
    }

    pub fn ten_degrees(self) -> (f32, f32, f32) {
        match self {
            Observer::A => (111.144, 100.0, 35.200),
            Observer::B => (99.178, 100.0, 84.3493),
            Observer::C => (97.285, 100.0, 116.145),
            Observer::D50 => (96.720, 100.0, 81.427),
            Observer::D55 => (95.799, 100.0, 90.926),
            Observer::D65 => (94.811, 100.0, 107.304),
            Observer::D75 => (94.416, 100.0, 120.641),
            Observer::E => (100.0, 100.0, 100.0),
            Observer::F1 => (94.791, 100.0, 103.191),
            Observer::F2 => (103.280, 100.0, 69.026),
            Observer::F3 => (108.968, 100.0, 51.965),
            Observer::F4 => (114.961, 100.0, 40.963),
            Observer::F5 => (93.369, 100.0, 98.636),
            Observer::F6 => (102.148, 100.0, 62.074),
            Observer::F7 => (95.792, 100.0, 107.687),
            Observer::F8 => (97.115, 100.0, 81.135),
            Observer::F9 => (102.116, 100.0, 67.826),
            Observer::F10 => (99.001, 100.0, 83.134),
            Observer::F11 => (103.866, 100.0, 65.627),
            Observer::F12 => (111.428, 100.0, 40.353),
        }
    }
}

impl From<u32> for Observer {
    fn from(value: u32) -> Self {
        match value {
            0 => Observer::A,
            1 => Observer::B,
            2 => Observer::C,
            3 => Observer::D50,
            4 => Observer::D55,
            5 => Observer::D65,
            6 => Observer::D75,
            7 => Observer::E,
            8 => Observer::F1,
            9 => Observer::F2,
            10 => Observer::F3,
            11 => Observer::F4,
            12 => Observer::F5,
            13 => Observer::F6,
            14 => Observer::F7,
            15 => Observer::F8,
            16 => Observer::F9,
            17 => Observer::F10,
            18 => Observer::F11,
            19 => Observer::F12,
            //default to D65
            _ => Observer::D65,
        }
    }
}
