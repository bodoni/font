#[test]
fn draw() {
    let font = &::setup().fonts[0];
    for code in b'a'..(b'z' + 1) {
        font.case.draw(code as char).unwrap().unwrap();
    }
}
