//! tegen is a Rust library designed to generate fast text from a template.
//! # Example
//! ```rs
//! use tegen::tegen::TextGenerator;

//! fn main() {
//!     let tg = TextGenerator::new();

//!     // Generate text from a given template
//!     println!("{}", tg.generate("{Hello|Greetings|Salutations}, {World|Reality}!"));

//!     // You can even nest templates!
//!     println!("{}", tg.generate("{Good {night|morning|evening|day}|Hello|Greetings|Howdy|What's up}, {friend|mate}! {How are you|How's it going}?"));
//! }
//! ```
pub mod tegen;
