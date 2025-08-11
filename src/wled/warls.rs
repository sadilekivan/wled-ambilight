use super::{ProtocolId, Strip, StripConfig};
use glam::U8Vec3;

// Max 255
// Store a cache of colors in memory since we can update leds with a index
#[derive(Debug, Clone)]
pub struct WARLS(Vec<U8Vec3>);

impl WARLS {
    pub fn new(cache_size: usize) -> Self {
        Self(vec![Default::default(); cache_size])
    }
}

impl ProtocolId for WARLS {
    fn protocol_id(&self) -> u8 {
        1
    }
}

impl From<StripConfig> for WARLS {
    fn from(value: StripConfig) -> Self {
        WARLS::new(value.total_wled_count() as usize)
    }
}

impl Strip<WARLS> {
    const WLED_SIZE: usize = 4;

    pub fn update_strip(&mut self, data: Vec<U8Vec3>) -> std::io::Result<usize> {
        let mut packet = Vec::with_capacity(
            self.strip_config.total_wled_count() as usize * Self::WLED_SIZE + Self::HEADER_SIZE,
        );
        packet.push(self.protocol.protocol_id());
        packet.push(1);
        for (i, color) in data.iter().enumerate() {
            if self.protocol.0.is_empty() {}
            let cached_color = &mut self.protocol.0[i];
            if *color != *cached_color {
                *cached_color = *color;

                packet.push(i as u8);
                packet.push(color[2]);
                packet.push(color[1]);
                packet.push(color[0]);
            }
        }

        // println!(
        //     "Updating {} WLEDs",
        //     (packet.len() - Self::HEADER_SIZE) / Self::WLED_SIZE
        // );
        self.socket.send(packet.as_slice())
    }
}
