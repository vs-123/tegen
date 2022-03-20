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
        self.scan_and_replace(text)
    }

    fn get_random_part(&self, text: &str, rng: &mut ThreadRng) -> String {
        let mut open_level = 0;
        let mut last_pos = 0;
        let mut is_ignore = false;

        let mut parts = Vec::<String>::with_capacity(text.len());

        for (i, c) in text.chars().enumerate() {
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
                parts.push(text[last_pos..i].to_string());
                last_pos = i + 1;
            }
        }

        parts.push(text[last_pos..].to_string());
        let rng = rng.gen_range(0..parts.len());
        parts[rng].clone()
    }

    fn scan_and_replace(&self, text: &str) -> String {
        let mut start_safe_pos = 0;
        let mut start_pos = 0;
        let mut end_pos = 0;
        let mut open_level = 0;
        let mut is_find = false;

        let mut result = String::new();

        // Create rng here, so there isn't a random number generator constructed every call to get_random_part.
        let mut rng = rand::thread_rng();
        for (i, c) in text.chars().enumerate() {
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

                    result.push_str(&self.scan_and_replace(
                        &self.get_random_part(&text[(start_pos + 1)..end_pos], &mut rng),
                    ));
                }
                continue;
            }
        }

        if !is_find {
            return text.to_string();
        }
        result.push_str(&text[(end_pos + 1)..]);

        result
    }
}
