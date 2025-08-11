#[derive(Debug, PartialEq)]
enum LEDPlacement {
    /// Example with 14 LEDs in a 9:4 ratio (in characters)
    /// o o o o o
    /// o       o
    /// o       o
    /// o o o o o
    CommonCorner, // Use ratio of these -1 on both axis or -2 on one axis as LED cutting points
    /// Example with 12 LEDs in a 9:4 ratio (in characters)
    ///  o o o o
    /// o       o
    /// o       o
    ///  o o o o
    EmptyCorner, // Use ratio of these as LED cutting points
}

fn split_leds(ratio: Vec2, led_strip_count: u32, mode: LEDPlacement) -> Vec2 {
    let led_strip_count = led_strip_count
        + match mode {
            LEDPlacement::CommonCorner => 4, // Add 4 corner LEDs for the ratio calculation, physically they are the same LEDs
            LEDPlacement::EmptyCorner => 0,
        };

    let total = ratio.element_sum();

    let per_led = led_strip_count as f32 / 2.0 / total;

    (ratio * per_led).round()
}

fn spread_led_over_pixels(led_count: u32, pixel_count: u32) -> Vec<u32> {
    let pixel_per_led = pixel_count as f32 / led_count as f32;
    let mut pixel_arr: Vec<u32> = Vec::with_capacity(led_count as usize);

    for i in 0..led_count {
        pixel_arr.push(((i as f32 + 0.5) * pixel_per_led).round() as u32);
    }

    pixel_arr
}
