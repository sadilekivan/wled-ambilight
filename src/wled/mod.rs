use glam::{U8Vec3, UVec2, UVec3};
use scrap::Frame;

pub mod strip;
pub mod protocol;

pub type U8RGB = U8Vec3;

pub trait SampleToColor {
    fn create_color(frame: &Frame, resolution: UVec2, sample: ScreenSample) -> U8RGB;
}

pub struct MeanAverage;

impl SampleToColor for MeanAverage {
    fn create_color(frame: &Frame, resolution: UVec2, sample: ScreenSample) -> U8RGB {
        let v = sample.get_color_v(frame, resolution);
        let x = v.into_iter().fold((UVec3::default(), 0), |acc, item| {
            (acc.0 + item.as_uvec3(), acc.1 + 1)
        });
        let avg = x.0 / x.1;
        avg.as_u8vec3()
    }
}

pub struct CenterPixel;

impl SampleToColor for CenterPixel {
    fn create_color(frame: &Frame, resolution: UVec2, sample: ScreenSample) -> U8RGB {
        get_frame_pixel_color(frame, resolution, sample.center_pixel())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenSample {
    pub b0: UVec2, // Min boundary
    pub b1: UVec2, // Max boundary
}

fn get_frame_pixel_color(frame: &Frame, resolution: UVec2, p: UVec2) -> U8RGB {
    let i = resolution.x * p.y + p.x;
    let i = i as usize * 4;
    U8RGB::from_slice(&frame[i..i + 4])
}

impl ScreenSample {
    pub fn delta(&self) -> UVec2 {
        self.b1 - self.b0
    }

    pub fn center_pixel(&self) -> UVec2 {
        self.b0 + self.delta() / 2
    }

    pub fn get_color_v(&self, frame: &Frame, resolution: UVec2) -> Vec<U8RGB> {
        let mut v = vec![];
        for x in self.b0.x..self.b1.x {
            for y in self.b0.y..self.b1.y {
                v.push(get_frame_pixel_color(frame, resolution, UVec2::new(x, y)));
            }
        }
        v
    }
}
