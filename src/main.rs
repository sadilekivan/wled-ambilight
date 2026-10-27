use glam::*;
use std::{
    error::Error,
    net::SocketAddrV4,
    thread::{self},
    time::{Duration, Instant},
};
mod wled;

use clap::*;

use crate::wled::{
    protocol,
    strip::{Strip, StripConfig},
    MeanAverage, SampleToColor, ScreenSample,
};

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(num_args = 2)]
    v: Vec<u32>,
}

fn worker(
    display: scrap::Display,
    target_frame_rate: u32,
    device_addr: SocketAddrV4,
) -> Result<(), Box<dyn Error>> {
    let mut strip = Strip::<protocol::WARLS>::new(StripConfig::new(
        UVec2::new(display.width() as u32, display.height() as u32),
        UVec4::new(42, 18, 41, 18),
        UVec4::new(450, 250, 450, 250),
        false,
        18 + 41 - 6,
    ));
    strip.connect(device_addr);

    let sample_v: Vec<ScreenSample> = strip.strip_config.to_samples();

    let resolution = strip.strip_config.screen_size;
    let mut c = scrap::Capturer::new(display)?;
    let mut it = std::iter::from_fn(|| {
        let frame = loop {
            if let Ok(frame) = c.frame() {
                break frame;
            }
        };

        let data: Vec<U8Vec3> = sample_v
            .iter()
            .map(|sample| MeanAverage::create_color(&frame, resolution, *sample))
            .collect();

        Some(data)
    });

    let dt = Duration::from_secs(1) / (target_frame_rate);
    loop {
        let now = Instant::now();
        let next = now + dt;

        let packet = it.next().unwrap();
        strip.update_strip(packet)?;

        thread::sleep(next - Instant::now());
        //println!("Frame time: {:.2?}", now.elapsed());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    ctrlc::set_handler(|| std::process::exit(0))?;
    env_logger::init();

    let _args = Args::parse();
    let display = scrap::Display::primary()?;
    worker(display, 60, "192.168.1.188:21324".parse()?)
}
