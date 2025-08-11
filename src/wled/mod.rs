use glam::{UVec2, UVec4, Vec4};
use std::net::{SocketAddrV4, UdpSocket};

pub mod drgb;
pub mod warls;

pub trait ProtocolId {
    fn protocol_id(&self) -> u8;
}

pub struct Strip<P> {
    pub strip_config: StripConfig,
    protocol: P,
    socket: UdpSocket,
}

impl<P> Strip<P>
where
    P: ProtocolId + From<StripConfig>,
{
    const HEADER_SIZE: usize = 2;

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

    pub fn create_segment_v(&self) -> Vec<LEDSegment> {
        let pixel_per_wled = self.pixel_per_wled();

        let mut segment_v = Vec::new();

        fn segment_line(segment_v: &mut Vec<LEDSegment>, led_count: u32, direction: UVec4) {
            for i in 0..led_count {}
        }

        // Top
        let offset = UVec2::new(self.edge_pixel_offset.x / 2, 0);
        for i in 0..self.led_count.x {
            let r = LEDSegment {
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
            //dbg_wled(&screen, &r);
        }

        // Right
        let offset = UVec2::new(
            self.screen_size.x - pixel_per_wled.y.round() as u32,
            self.edge_pixel_offset.y / 2,
        );
        for i in 0..self.led_count.y {
            let r = LEDSegment {
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
            //dbg_wled(&screen, &r);
        }

        // Bottom
        let offset = UVec2::new(
            self.edge_pixel_offset.z / 2,
            self.screen_size.y - pixel_per_wled.z.round() as u32,
        );
        for i in (0..self.led_count.z).rev() {
            let r = LEDSegment {
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
            //dbg_wled(&screen, &r,);
        }

        // Left
        let offset = UVec2::new(0, self.edge_pixel_offset.w / 2);
        for i in (0..self.led_count.w).rev() {
            let r = LEDSegment {
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
            //dbg_wled(&screen, &r);
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

#[derive(Debug)]
pub struct LEDSegment {
    pub b0: UVec2,
    pub b1: UVec2,
}

impl LEDSegment {
    pub fn delta(&self) -> UVec2 {
        self.b1 - self.b0
    }

    pub fn center_pixel(&self) -> UVec2 {
        self.b0 + self.delta() / 2
    }

    // Samples in a square grid pattern
    pub fn square_pixel(&self, sample_size: usize) -> Vec<UVec2> {
        let mut v = Vec::with_capacity(sample_size);

        v
    }

    /*/// Evenly spread LEDSegments over the pixels
    fn from_pixel_strip(screen: impl ScreenOps, pixel_count: f32, led_count: f32) -> Vec<Self> {
        let mut segment_v: Vec<Self> = Vec::with_capacity(led_count as usize);
        let pixel_per_led = pixel_count / led_count;

        for i in 0..led_count as usize {
            let i = i as f32;
            let segment = Self {
                b0: Vec2::new((i * pixel_per_led).round(), 0.),
                b1: Vec2::new(((i + 1.0) * pixel_per_led).round() - 1., pixel_per_led.round() - 1.),
                screen,
            };
            segment_v.push(segment);
        }

        segment_v
    }*/
}

// fn dbg_wled(screen: &impl ScreenOps, wled: &LEDSegment) {
//     let color = screen.get_pixel(wled.center_pixel());
//     let LEDSegment { b0, b1} = wled;

//     for x in b0.x as u32..b1.x as u32 {
//         for y in b0.y as u32..b1.y as u32 {
//             screen.set_pixel(Vec2::new(x as f32, y as f32), color);
//         }
//     }

//     // Rectangle
//     for pixel in b0.x as u32..b1.x as u32 {
//         screen.set_pixel(Vec2::new(pixel as f32, b0.y), color::GREEN);
//         screen.set_pixel(Vec2::new(pixel as f32, b1.y), color::GREEN);
//     }

//     for pixel in b0.y as u32..b1.y as u32 {
//         screen.set_pixel(Vec2::new(b0.x, pixel as f32), color::GREEN);
//         screen.set_pixel(Vec2::new(b1.x, pixel as f32), color::GREEN);
//     }

//     // See where we draw from to where
//     //thread::sleep(Duration::from_secs_f32(0.01));
// }
