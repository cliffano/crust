//! Run with: cargo run --example display

use rustcliexample::display::Display;

fn main() {
    let display = Display::new("examples/rustcliexample.yaml").expect("failed to load config");
    let text = display.format(false, "lower");
    println!("{text}");
}
