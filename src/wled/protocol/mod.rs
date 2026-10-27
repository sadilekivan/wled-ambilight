use crate::wled::U8RGB;
mod drgb;
mod warls;

pub use warls::*;

pub trait Protocol {
    const HEADER_SIZE: usize = 2;
    const PROTOCOL_ID: u8;
    const SINGLE_WLED_SIZE: usize;

    fn create_packet(&mut self, data: Vec<U8RGB>) -> Vec<u8>;
}
