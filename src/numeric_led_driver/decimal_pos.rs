#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DecimalPos {
    #[default]
    None,
    Thousands,
    Hundreds,
    Tens,
    Ones,
}