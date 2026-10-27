use std::net::{SocketAddrV4, UdpSocket};

use glam::{UVec2, UVec4, Vec4};

use crate::wled::{protocol::Protocol, ScreenSample, U8RGB};

pub struct Strip<P> {
    pub strip_config: StripConfig,
    protocol: P,
    socket: UdpSocket,
}

impl<P> Strip<P>
where
    P: Protocol + From<StripConfig>,
{
    pub fn new(strip_config: StripConfig) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("This should have worked :/");
        let protocol = P::from(strip_config);
        Self {
            strip_config,
            protocol,
            socket,
        }
    }

    pub fn connect(&self, ip_address: SocketAddrV4) {
        self.socket
            .connect(ip_address)
            .expect("Cannot connect to WLED strip");
    }

    pub fn update_strip(&mut self, data: Vec<U8RGB>) -> std::io::Result<usize> {
        let packet = self.protocol.create_packet(data);
        self.socket.send(packet.as_slice())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StripConfig {
    pub screen_size: UVec2,
    pub led_count: UVec4,
    pub edge_pixel_offset: UVec4,
    pub clockwise: bool,
    pub shift: u32,
}

impl StripConfig {
    pub fn new(
        screen_size: UVec2,
        led_count: UVec4,
        edge_pixel_offset: UVec4,
        clockwise: bool,
        shift: u32,
    ) -> Self {
        Self {
            screen_size,
            led_count,
            edge_pixel_offset,
            clockwise,
            shift,
        }
    }

    pub fn total_wled_count(&self) -> u32 {
        self.led_count.element_sum()
    }

    pub fn pixel_per_wled(&self) -> Vec4 {
        let screen_size = UVec4::new(
            self.screen_size.x,
            self.screen_size.y,
            self.screen_size.x,
            self.screen_size.y,
        );
        (screen_size.as_vec4() - self.edge_pixel_offset.as_vec4()) / self.led_count.as_vec4()
    }

    pub fn to_samples(self) -> Vec<ScreenSample> {
        let pixel_per_wled = self.pixel_per_wled();

        let mut segment_v = Vec::new();

        // fn segment_line(segment_v: &mut [ScreenSample], led_count: u32, direction: UVec4) {
        //     todo!();
        // }

        // Top
        let offset = UVec2::new(self.edge_pixel_offset.x / 2, 0);
        for i in 0..self.led_count.x {
            let r = ScreenSample {
                b0: offset
                    + (UVec2::new(i, 0).as_vec2() * pixel_per_wled.x)
                        .round()
                        .as_uvec2(),
                b1: offset
                    + (UVec2::new(i + 1, 1).as_vec2() * pixel_per_wled.x)
                        .round()
                        .as_uvec2()
                    - 1,
            };

            segment_v.push(r);
        }

        // Right
        let offset = UVec2::new(
            self.screen_size.x - pixel_per_wled.y.round() as u32,
            self.edge_pixel_offset.y / 2,
        );
        for i in 0..self.led_count.y {
            let r = ScreenSample {
                b0: offset
                    + (UVec2::new(0, i).as_vec2() * pixel_per_wled.y)
                        .round()
                        .as_uvec2(),
                b1: offset
                    + (UVec2::new(1, i + 1).as_vec2() * pixel_per_wled.y)
                        .round()
                        .as_uvec2()
                    - 1,
            };

            segment_v.push(r);
        }

        // Bottom
        let offset = UVec2::new(
            self.edge_pixel_offset.z / 2,
            self.screen_size.y - pixel_per_wled.z.round() as u32,
        );
        for i in (0..self.led_count.z).rev() {
            let r = ScreenSample {
                b0: offset
                    + (UVec2::new(i, 0).as_vec2() * pixel_per_wled.z)
                        .round()
                        .as_uvec2(),
                b1: offset
                    + (UVec2::new(i + 1, 1).as_vec2() * pixel_per_wled.z)
                        .round()
                        .as_uvec2()
                    - 1,
            };

            segment_v.push(r);
        }

        // Left
        let offset = UVec2::new(0, self.edge_pixel_offset.w / 2);
        for i in (0..self.led_count.w).rev() {
            let r = ScreenSample {
                b0: offset
                    + (UVec2::new(0, i).as_vec2() * pixel_per_wled.w)
                        .round()
                        .as_uvec2(),
                b1: offset
                    + (UVec2::new(1, i + 1).as_vec2() * pixel_per_wled.w)
                        .round()
                        .as_uvec2()
                    - 1,
            };

            segment_v.push(r);
        }

        if self.clockwise {
            segment_v.rotate_right(self.shift as usize);
        } else {
            segment_v.reverse();
            segment_v.rotate_left(self.shift as usize);
        }

        segment_v
    }
}
