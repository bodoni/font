#[test]
fn draw() {
    let font = &::setup().fonts[0];
    let _ = font.case.draw('a').unwrap().unwrap();
}
