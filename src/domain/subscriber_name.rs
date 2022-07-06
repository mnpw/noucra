use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: &str) -> Result<Self, String> {
        // check if just whitespace or empty
        let is_empty = name.trim().is_empty();

        // check if too long
        let is_too_long = name.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = name
            .chars()
            .any(|char| forbidden_characters.contains(&char));

        if is_empty || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", name))
        } else {
            Ok(Self(name.to_string()))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest() {
        let name = "a".repeat(256);
        claim::assert_ok!(SubscriberName::parse(&name));
    }

    #[test]
    fn too_long() {
        let name = "a".repeat(257);
        claim::assert_err!(SubscriberName::parse(&name));
    }

    #[test]
    fn just_whitespace() {
        let name = " ".to_string();
        claim::assert_err!(SubscriberName::parse(&name));
    }

    #[test]
    fn empty() {
        let name = "".to_string();
        claim::assert_err!(SubscriberName::parse(&name));
    }

    #[test]
    fn forbidden() {
        for name in ['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            claim::assert_err!(SubscriberName::parse(&name));
        }
    }

    #[test]
    fn valid_name() {
        let name = "John Doe";
        claim::assert_ok!(SubscriberName::parse(&name));
    }
}
