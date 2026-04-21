use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Self {
        const MAXIMUM_NAME_LENGTH: usize = 256;
        const FORBBIDEN_CHARS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        // Should not be empty
        let is_empty = name.trim().is_empty();

        // Should be shorter than MAXIMUM_NAME_LENGTH
        let has_invalid_length = name.graphemes(true).count() > MAXIMUM_NAME_LENGTH;

        // Should not contain any forbbiden characters
        let has_invalid_chars = name.chars().any(|c| FORBBIDEN_CHARS.contains(&c));

        if is_empty || has_invalid_chars || has_invalid_length {
            panic!("Invalid name, can not parse to SubscriberName");
        }

        Self(name)
    }

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
