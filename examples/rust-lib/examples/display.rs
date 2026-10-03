//! Run with: cargo run --example display

use rustlibexample::display::Display;

fn main() {
    let display = Display::new("examples/rustlibexample.yaml").expect("failed to load config");
    let text = display.format(false, "lower");
    println!("{text}");
}
