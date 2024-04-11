use embassy_rp::gpio::AnyPin;

pub struct NumericLedPins {
    pub(super) dig_1: AnyPin,
    pub(super) dig_2: AnyPin,
    pub(super) dig_3: AnyPin,
    pub(super) dig_4: AnyPin,
    pub(super) a: AnyPin,
    pub(super) b: AnyPin,
    pub(super) c: AnyPin,
    pub(super) d: AnyPin,
    pub(super) e: AnyPin,
    pub(super) f: AnyPin,
    pub(super) g: AnyPin,
    pub(super) dp: AnyPin,
}

impl NumericLedPins {
    pub fn new<
        TDig1: Into<AnyPin>,
        TDig2: Into<AnyPin>,
        TDig3: Into<AnyPin>,
        TDig4: Into<AnyPin>,
        TA: Into<AnyPin>,
        TB: Into<AnyPin>,
        TC: Into<AnyPin>,
        TD: Into<AnyPin>,
        TE: Into<AnyPin>,
        TF: Into<AnyPin>,
        TG: Into<AnyPin>,
        TDp: Into<AnyPin>,
    >(
        dig_1: TDig1,
        dig_2: TDig2,
        dig_3: TDig3,
        dig_4: TDig4,
        a: TA,
        b: TB,
        c: TC,
        d: TD,
        e: TE,
        f: TF,
        g: TG,
        dp: TDp,
    ) -> Self {
        Self {
            dig_1: dig_1.into(),
            dig_2: dig_2.into(),
            dig_3: dig_3.into(),
            dig_4: dig_4.into(),
            a: a.into(),
            b: b.into(),
            c: c.into(),
            d: d.into(),
            e: e.into(),
            f: f.into(),
            g: g.into(),
            dp: dp.into(),
        }
    }
}
