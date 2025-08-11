use glam::*;
use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};
use wled::warls::WARLS;
mod wled;

use clap::*;

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(num_args = 2)]
    v: Vec<u32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    ctrlc::set_handler(|| std::process::exit(0))?;
    let args = Args::parse();
    let display = scrap::Display::primary()?;

    let mut strip = wled::Strip::<WARLS>::new(wled::StripConfig::new(
        UVec2::new(display.width() as u32, display.height() as u32),
        UVec4::new(42, 18, 41, 18),
        UVec4::new(450, 250, 450, 250),
        false,
        18 + 41 - 6,
    ));
    strip.connect("192.168.2.217:21324".parse()?);

    let pixel_v: Vec<UVec2> = strip
        .strip_config
        .create_segment_v()
        .iter()
        .map(wled::LEDSegment::center_pixel)
        .collect();

    let mut c = scrap::Capturer::new(display)?;

    let target_frame_rate = 60;
    let dt = Duration::from_secs(1) / (target_frame_rate);

    let y = strip.strip_config.screen_size.y as usize;
    let mut it = std::iter::from_fn(|| {
        let frame = loop {
            if let Ok(frame) = c.frame() {
                break frame;
            }
        };

        let stride = frame.len() / y;

        let data: Vec<U8Vec3> = pixel_v
            .iter()
            .map(|p| {
                let i = stride * p.y as usize + 4 * p.x as usize;
                U8Vec3::from_slice(&frame[i..i + 4])
            })
            .collect();

        Some(data)
    });

    loop {
        let now = Instant::now();
        let next = now + dt;

        let packet = it.next().unwrap();
        strip.update_strip(packet)?;

        thread::sleep(next - Instant::now());
        //println!("Frame time: {:.2?}", now.elapsed());
    }
}
