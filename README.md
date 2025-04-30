# tegen
tegen is a Rust library designed to generate fast text from a template. (heavily inspired from [liderman's text generator](https://github.com/liderman/text-generator))

# Usage
```rs
use tegen::tegen::TextGenerator;

fn main() {
    let tg = TextGenerator::new();

    // Generate text from a given template
    println!("{}", tg.generate("{Hello|Greetings|Salutations}, {World|Reality}!"));

    // You can even nest templates!
    println!("{}", tg.generate("{Good {night|morning|evening|day}|Hello|Greetings|Howdy|What's up}, {friend|mate}! {How are you|How's it going}?"));
}
```

# Features
<ul>
  <li>Faster than cup noodles</li>
  <li>Does not use regular expressions</li>
  <li>Very user friendly</li>
  <li>Included examples which can be run by using `cargo run --example [example_name]`</li>
</ul>

# Goals
The main goal of this library is to generate text from a given template as fast as possible.
Other than that, the other goals of this library are:
<ul>
  <li>Easy to use</li>
  <li>Easy to extend</li>
  <li>Easy to understand</li>
  <li>Easy to maintain</li>
</ul>

# How to contribute
<ul>
  <li>Create an issue for bug reports or feature suggestions</li>
  <li>Create a pull request of a new feature or a bug fix describing the changes it will make</li>
</ul>