use rustlibexample::display::Display;

#[test]
fn construct_display_and_format_with_lower_transformation_and_no_reverse() {
    let display = Display::new("examples/rustlibexample.yaml").unwrap();
    let text = display.format(false, "lower");
    assert_eq!(text, "hello world");
}

#[test]
fn construct_display_and_format_with_upper_transformation_and_reverse() {
    let display = Display::new("examples/rustlibexample.yaml").unwrap();
    let text = display.format(true, "upper");
    assert_eq!(text, "DLROW OLLEH");
}
