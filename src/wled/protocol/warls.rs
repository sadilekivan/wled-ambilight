use glam::U8Vec3;

use crate::wled::{protocol::Protocol, strip::StripConfig};

// Max 255
// Store a cache of colors in memory since we can update leds with a index
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub struct WARLS(Vec<U8Vec3>);

impl WARLS {
    pub fn new(cache_size: usize) -> Self {
        Self(vec![Default::default(); cache_size])
    }
}

impl Protocol for WARLS {
    const PROTOCOL_ID: u8 = 1;
    const SINGLE_WLED_SIZE: usize = 4;

    fn create_packet(&mut self, data: Vec<super::U8RGB>) -> Vec<u8> {
        let mut packet =
            Vec::with_capacity(data.len() * Self::SINGLE_WLED_SIZE + Self::HEADER_SIZE);
        packet.push(Self::PROTOCOL_ID);
        packet.push(1);
        for (i, color) in data.iter().enumerate() {
            let cached_color = &mut self.0[i];
            if *color != *cached_color {
                *cached_color = *color;

                packet.push(i as u8);
                packet.push(color[2]);
                packet.push(color[1]);
                packet.push(color[0]);
            }
        }

        let updated_led_count = (packet.len() - Self::HEADER_SIZE) / Self::SINGLE_WLED_SIZE;
        if updated_led_count > 0 {
            log::debug!("Streaming {} leds", updated_led_count);
        }
        packet
    }
}

impl From<StripConfig> for WARLS {
    fn from(value: StripConfig) -> Self {
        WARLS::new(value.total_wled_count() as usize)
    }
}
