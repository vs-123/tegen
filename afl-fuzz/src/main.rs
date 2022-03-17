#[macro_use]
extern crate afl;
use tegen::tegen::TextGenerator;
fn main() {
    fuzz!(|data: &[u8]| {
        let tg = TextGenerator::new();
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = tg.generate(s);
        }
    });
}