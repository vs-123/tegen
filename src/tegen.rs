use std::borrow::Cow;

use rand::{prelude::ThreadRng, Rng};

/// The `TextGenerator` struct, used for generating text from a template.
pub struct TextGenerator {
    start_c: char,
    end_c: char,
    sep: char,
}

impl Default for TextGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl TextGenerator {
    /// Creates a new `TextGenerator` struct.
    pub fn new() -> Self {
        Self {
            start_c: '{',
            end_c: '}',
            sep: '|',
        }
    }

    /// Generates text from the given template.
    /// A template starts with `{` and ends with `}` and can contain `|` characters to separate different options.
    /// For example, `{Hello|Salutations}, {World|Reality}!`
    pub fn generate(&self, text: &str) -> String {
        let mut rng = rand::thread_rng();
        self.scan_and_replace(text.into(), &mut rng).into_owned()
    }

    /// Given text of a template without outer brackets, chooses and returns one of template's options.
    fn get_random_part<'a>(&self, text: &'a str, rng: &mut ThreadRng) -> &'a str {
        let mut open_level = 0;
        let mut last_pos = 0;
        let mut is_ignore = false;

        let mut parts = Vec::<&'a str>::new();

        for (i, c) in text.char_indices() {
            if c == self.start_c {
                open_level += 1;
                is_ignore = true;
                continue;
            }

            if c == self.end_c {
                open_level -= 1;
                if open_level == 0 {
                    is_ignore = false;
                }
                continue;
            }

            if is_ignore {
                continue;
            }

            if c == self.sep {
                parts.push(&text[last_pos..i]);
                last_pos = i + 1;
            }
        }

        parts.push(&text[last_pos..]);
        let rng = rng.gen_range(0..parts.len());
        parts[rng]
    }

    fn scan_and_replace<'a>(&self, text: Cow<'a, str>, rng: &mut ThreadRng) -> Cow<'a, str> {
        let mut start_safe_pos = 0;
        let mut start_pos = 0;
        let mut end_pos = 0;
        let mut open_level = 0;
        let mut is_find = false;

        let mut result = String::new();

        for (i, c) in text.char_indices() {
            if c == self.start_c {
                if open_level == 0 {
                    start_pos = i;
                    result.push_str(&text[start_safe_pos..start_pos]);
                }

                open_level += 1;
                continue;
            }

            if c == self.end_c {
                open_level -= 1;
                if open_level == 0 {
                    is_find = true;
                    end_pos = i;

                    start_safe_pos = end_pos + 1;

                    let template_slice = &text[(start_pos + 1)..end_pos];
                    let text_choice = self.get_random_part(template_slice, rng);
                    result.push_str(&self.scan_and_replace(text_choice.into(), rng));
                }
                continue;
            }
        }

        if !is_find {
            return text;
        }
        result.push_str(&text[(end_pos + 1)..]);

        Cow::Owned(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_template() {
        let strings = vec![
            "hello world",
            "",
            "a",
            "123",
            "{no closing",
            "no opening}",
            "}{",
            "{",
            "}",
            "{{{{{{{{{{{{{{{",
            "|",
        ];

        for expected in strings {
            let result = TextGenerator::new().generate(expected);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn simple_single_template() {
        let template = "{hello}";
        let expected = "hello";
        let result = TextGenerator::new().generate(template);
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_single_template_special_character() {
        let template = "{y̆es}";
        let expected = "y̆es";
        let result = TextGenerator::new().generate(template);
        assert_eq!(result, expected);
    }

    #[test]
    fn general_deterministic_templates() {
        let template_expected_pairs = vec![
            ("Hello", "Hello"),
            ("{}", ""),
            ("Before {During} After\n ", "Before During After\n "),
        ];
        for (template, expected) in template_expected_pairs {
            let result = TextGenerator::new().generate(template);
            assert_eq!(result, expected);
        }
    }
}
