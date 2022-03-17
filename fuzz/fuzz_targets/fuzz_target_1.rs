#![no_main]
use libfuzzer_sys::fuzz_target;
use tegen::tegen::TextGenerator;

fuzz_target!(|data: &[u8]| {
    let tg = TextGenerator::new();
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = tg.generate(s);
    }
});
