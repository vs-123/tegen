use tegen::tegen::TextGenerator;

fn main() {
    let tg = TextGenerator::new();
    println!(
        "{}",
        tg.generate("{Hello|Greetings|Salutations}, {World|Reality}!")
    );
}
