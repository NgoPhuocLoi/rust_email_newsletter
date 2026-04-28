use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<Self, String> {
        if email.validate_email() {
            Ok(SubscriberEmail(email))
        } else {
            Err(format!("{} is not a valid subscriber email.", email))
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
    use claim::{assert_err, assert_ok};
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use quickcheck::Arbitrary;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use crate::domain::SubscriberEmail;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let seed = u64::arbitrary(g);
            let mut rng = StdRng::seed_from_u64(seed);
            let email: String = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }

    #[test]
    fn empty_email_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn missing_subject_email_is_rejected() {
        let email = "@gmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn missing_at_char_email_is_rejected() {
        let email = "addgmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn valid_emails_are_accepted() {
        let email: String = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }

    #[test]
    fn missing_domain_is_rejected() {
        let email = "user@".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn whitespace_only_email_is_rejected() {
        let email = "   ".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn multiple_at_signs_are_rejected() {
        let email = "user@@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_with_spaces_is_rejected() {
        let email = "user name@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_with_valid_subdomain_is_accepted() {
        let email = "user@mail.example.com".to_string();
        assert_ok!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_with_plus_sign_is_accepted() {
        let email = "user+tag@example.com".to_string();
        assert_ok!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_with_dots_in_local_part_is_accepted() {
        let email = "first.last@example.com".to_string();
        assert_ok!(SubscriberEmail::parse(email));
    }
}
