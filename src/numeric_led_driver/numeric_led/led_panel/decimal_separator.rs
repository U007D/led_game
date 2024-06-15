#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DecimalSeparator {
    #[default]
    None,
    Thousands,
    Hundreds,
    Tens,
    Ones,
}