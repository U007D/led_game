use crate::numeric_led_driver::DecimalPos;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LedPanel {
    One,
    Two,
    Three,
    Four,
}

impl LedPanel {
    pub fn has_dp(&self, decimal_pos: DecimalPos) -> bool {
        use LedPanel as Lp;
        use DecimalPos as Dp;
        match self {
            Lp::One => decimal_pos == Dp::Thousands,
            Lp::Two => decimal_pos == Dp::Hundreds,
            Lp::Three => decimal_pos == Dp::Tens,
            Lp::Four => decimal_pos == Dp::Ones,
        }
    }
}


