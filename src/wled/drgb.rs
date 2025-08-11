use super::{ProtocolId, Strip, StripConfig};
use glam::U8Vec3;

// Max 490
#[derive(Debug, Default, Clone, Copy)]
pub struct DRGB;

impl ProtocolId for DRGB {
    fn protocol_id(&self) -> u8 {
        2
    }
}

impl From<StripConfig> for DRGB {
    fn from(_value: StripConfig) -> Self {
        DRGB
    }
}

impl Strip<DRGB> {
    const WLED_SIZE: usize = 3;

    pub fn update_strip(&mut self, data: Vec<U8Vec3>) -> std::io::Result<usize> {
        let mut packet = Vec::with_capacity(
            self.strip_config.total_wled_count() as usize * Self::WLED_SIZE + Self::HEADER_SIZE,
        );
        packet.push(self.protocol.protocol_id());
        packet.push(1);
        for color in data {
            packet.push(color[2]);
            packet.push(color[1]);
            packet.push(color[0]);
        }

        println!(
            "Updating {} WLEDs",
            (packet.len() - Self::HEADER_SIZE) / Self::WLED_SIZE
        );
        self.socket.send(packet.as_slice())
    }
}
