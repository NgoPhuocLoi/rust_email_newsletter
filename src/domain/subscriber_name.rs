use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<Self, String> {
        const MAXIMUM_NAME_LENGTH: usize = 256;
        const FORBBIDEN_CHARS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        // Should not be empty
        let is_empty = name.trim().is_empty();

        // Should be shorter than MAXIMUM_NAME_LENGTH
        let has_invalid_length = name.graphemes(true).count() > MAXIMUM_NAME_LENGTH;

        // Should not contain any forbbiden characters
        let has_invalid_chars = name.chars().any(|c| FORBBIDEN_CHARS.contains(&c));

        if is_empty || has_invalid_chars || has_invalid_length {
            return Err(format!("{} is not a valid subscriber name.", name));
        }

        Ok(Self(name))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};

    use crate::domain::SubscriberName;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_257_grapheme_long_name_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn a_single_character_name_is_valid() {
        assert_ok!(SubscriberName::parse("A".to_string()));
    }

    #[test]
    fn a_normal_name_is_valid() {
        assert_ok!(SubscriberName::parse("John Doe".to_string()));
    }

    #[test]
    fn a_unicode_name_is_valid() {
        // Each emoji counts as one grapheme
        let name = "é à ü ñ".to_string();
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn an_empty_name_is_rejected() {
        assert_err!(SubscriberName::parse("".to_string()));
    }

    #[test]
    fn a_whitespace_only_name_is_rejected() {
        assert_err!(SubscriberName::parse("   ".to_string()));
    }

    #[test]
    fn names_containing_forbidden_characters_are_rejected() {
        const FORBIDDEN_CHARS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        for &ch in &FORBIDDEN_CHARS {
            let name = format!("Valid{ch}Name");
            assert_err!(
                SubscriberName::parse(name),
                "Expected Err for forbidden char: {ch}"
            );
        }
    }
}
