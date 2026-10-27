use crate::wled::{protocol::Protocol, strip::StripConfig, U8RGB};

// Max 490
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DRGB;

impl Protocol for DRGB {
    const PROTOCOL_ID: u8 = 2;
    const SINGLE_WLED_SIZE: usize = 3;

    fn create_packet(&mut self, data: Vec<U8RGB>) -> Vec<u8> {
        let mut packet =
            Vec::with_capacity(data.len() * Self::SINGLE_WLED_SIZE + Self::HEADER_SIZE);
        packet.push(Self::PROTOCOL_ID);
        packet.push(1);
        for color in data {
            packet.push(color[2]);
            packet.push(color[1]);
            packet.push(color[0]);
        }

        let updated_led_count = (packet.len() - Self::HEADER_SIZE) / Self::SINGLE_WLED_SIZE;
        if updated_led_count > 0 {
            log::debug!("Streaming {} leds", updated_led_count);
        }
        packet
    }
}

impl From<StripConfig> for DRGB {
    fn from(_value: StripConfig) -> Self {
        DRGB
    }
}
