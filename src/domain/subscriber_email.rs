use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    fn parse(email: &str) -> Result<Self, String> {
        if validate_email(email) {
            Ok(Self(email.to_string()))
        } else {
            Err(format!("{} is not a valid email format.", email))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[derive(Clone, Debug)]
    struct ValidEmailFixture(String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email: String = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    impl AsRef<str> for ValidEmailFixture {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(&email));
    }

    #[test]
    fn missing_domain_is_rejected() {
        let email = "testmailcom".to_string();
        assert_err!(SubscriberEmail::parse(&email));
    }

    #[test]
    fn missing_username_is_rejected() {
        let email = "@mail.com".to_string();
        assert_err!(SubscriberEmail::parse(&email));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_accepted(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.as_ref()).is_ok()
    }
}
