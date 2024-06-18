use crate::{GAME_CHANNEL, message::GameMsg as Gm};
use embassy_rp::gpio::{AnyPin, Input, Pull};

#[embassy_executor::task]
pub async fn button_driver(button_pin: AnyPin) -> ! {
    let mut button = Input::new(button_pin, Pull::Down);

    loop {
        button.wait_for_rising_edge().await;
        GAME_CHANNEL.send(Gm::ButtonDown).await;

        button.wait_for_falling_edge().await;
        GAME_CHANNEL.send(Gm::ButtonUp).await;
    }
}
