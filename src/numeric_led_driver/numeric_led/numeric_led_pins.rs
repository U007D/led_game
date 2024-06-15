use embassy_rp::peripherals;

pub struct NumericLedPins {
    pub(super) dig_1: peripherals::PIN_1,
    pub(super) dig_2: peripherals::PIN_2,
    pub(super) dig_3: peripherals::PIN_3,
    pub(super) dig_4: peripherals::PIN_4,
    pub(super) a: peripherals::PIN_5,
    pub(super) b: peripherals::PIN_6,
    pub(super) c: peripherals::PIN_7,
    pub(super) d: peripherals::PIN_8,
    pub(super) e: peripherals::PIN_9,
    pub(super) f: peripherals::PIN_10,
    pub(super) g: peripherals::PIN_11,
    pub(super) dp: peripherals::PIN_12,
}

impl NumericLedPins {
    pub fn new(
        dig_1: peripherals::PIN_1,
        dig_2: peripherals::PIN_2,
        dig_3: peripherals::PIN_3,
        dig_4: peripherals::PIN_4,
        a: peripherals::PIN_5,
        b: peripherals::PIN_6,
        c: peripherals::PIN_7,
        d: peripherals::PIN_8,
        e: peripherals::PIN_9,
        f: peripherals::PIN_10,
        g: peripherals::PIN_11,
        dp: peripherals::PIN_12,
    ) -> Self {
        Self {
            dig_1,
            dig_2,
            dig_3,
            dig_4,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            dp,
        }
    }
}
