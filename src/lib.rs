//! tegen is a Rust library designed to generate fast text from a template.
//! # Example
//! ```rs
//! use tegen::TextGenerator;
//! fn main() {
//!     let tg = TextGenerator::new();
//!     println!("{}", tg.generate("{Good {night|morning|evening|day}|Hello|Greetings|Howdy|What's up}, {friend|mate}! {How are you|How's it going}?".to_string()));
//! }
//! ```
pub mod tegen;

fn example() {
    let tg = TextGenerator::new();

    println!("{}", tg.generate("{Good {night|morning|evening|day}|Hello|Greetings|Howdy|What's up}, {friend|mate}! {How are you|How's it going}?".to_string()));
}
