# tegen
tegen is a Rust library designed to generate fast text from a template. (heavily inspired from [liderman's text generator](https://github.com/liderman/text-generator))

# Usage
```rs
use tegen::tegen::TextGenerator;

fn main() {
    let tg = TextGenerator::new();

    println!("{}", tg.generate("{Good {night|morning|evening|day}|Hello|Greetings|Howdy|What's up}, {friend|mate}! {How are you|How's it going}?".to_string()));
}
```

# Features
<ul>
  <li>Faster than cup noodles</li>
  <li>Does not use regular expressions</li>
  <li>Idk</li>
</ul>