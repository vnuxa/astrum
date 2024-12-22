use std::time::Duration;

use cosmic::iced::{time, Subscription};

use crate::astrum_core::app::main::{AstrumMessages, StringOrNum};

pub fn time_service_channel(requested_rate: u64) -> Subscription<AstrumMessages> {
    time::every(Duration::from_secs(requested_rate)).map(move |_| {
        AstrumMessages::SubscriptionRequest(
            (
                "time".to_string(),
                StringOrNum::Num(requested_rate as i64),
                "{}".to_string(),
            )

        )
    })
}
