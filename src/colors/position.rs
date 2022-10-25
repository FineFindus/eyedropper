/// Position of the alpha value for hex strings.
///
/// In most cases (for example hex strings in the browser) the alpha value is
/// the last two characters of the hex string. But in some cases it is the first two characters.
/// For example Android Color Values use this format.
///
/// Defaults to no alpha value
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum AlphaPosition {
    #[default]
    End,
    Start,
    None,
}

//Convert from U32. Needed for converting from the settings AdwComboRow, which use indexes for values.
impl From<u32> for AlphaPosition {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::End,
            2 => Self::Start,
            _ => Self::default(),
        }
    }
}
