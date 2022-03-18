use rand::Rng;

/// The `TextGenerator` struct, used for generating text from a template.
pub struct TextGenerator {
    start_c: char,
    end_c: char,
    sep: char,
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
        self.scan_and_replace(text.chars().collect::<Vec<char>>())
            .iter()
            .collect()
    }

    fn get_random_part(&self, text: Vec<char>) -> Vec<char> {
        let mut open_level = 0;
        let mut last_pos = 0;
        let mut is_ignore = false;

        let mut parts = Vec::<String>::with_capacity(text.len() + 1);

        // The capacity never surpasses text.len() + 1, because only after a series of conditionals is
        // parts pushed to. (Then one extra, for the last line, though I doubt every single character
        // is a separator).

        for (i, &c) in text.iter().enumerate() {
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
                parts.push(text[last_pos..i].into_iter().collect::<String>());
                last_pos = i + 1;
            }
        }

        parts.push(text[last_pos..].iter().collect::<String>());

        let mut rng = rand::thread_rng();
        parts[rng.gen_range(0..parts.len())].chars().collect()
    }

    fn scan_and_replace(&self, text: Vec<char>) -> Vec<char> {
        let mut start_safe_pos = 0;
        let mut start_pos = 0;
        let mut end_pos = 0;
        let mut open_level = 0;
        let mut is_find = false;

        let mut result = Vec::<char>::with_capacity(text.len());

        // The capacity never surpasses text.len() + 1, because only after a series of conditionals is
        // parts pushed to. (Then one extra, for the last line, though I doubt every single character
        // is a separator).

        for (i, &c) in text.iter().enumerate() {
            if c == self.start_c {
                if open_level == 0 {
                    start_pos = i;
                    result.append(
                        &mut text[start_safe_pos..start_pos].to_vec(),
                    );
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
                    result.append(
                        &mut self.scan_and_replace(
                            self.get_random_part(
                                text[(start_pos + 1)..end_pos].to_vec(),
                            ),
                        ),
                    );
                }
                continue;
            }
        }

        if !is_find {
            return text;
        }

        result.append(&mut text[(end_pos + 1)..].to_vec());

        result
    }
}
