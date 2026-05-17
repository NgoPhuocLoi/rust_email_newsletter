use std::time::Duration;

use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;

use crate::domain::SubscriberEmail;

pub struct EmailClient {
    http_client: Client,
    sender_email: SubscriberEmail,
    base_url: String,
    api_key: SecretString,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender_email: SubscriberEmail,
        timeout: Duration,
        api_key: SecretString,
    ) -> Self {
        Self {
            http_client: Client::builder()
                .timeout(timeout)
                .build()
                .expect("Failed to create new EmailClient"),
            base_url,
            sender_email,
            api_key,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/v3/smtp/email", self.base_url);
        let recipients = [EmailContact {
            email: recipient.as_ref(),
        }];
        let request_body = SendEmailRequest {
            sender: EmailContact {
                email: self.sender_email.as_ref(),
            },
            to: &recipients,
            subject,
            html_content,
        };

        self.http_client
            .post(&url)
            .header("api-key", self.api_key.expose_secret())
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    sender: EmailContact<'a>,
    to: &'a [EmailContact<'a>],
    subject: &'a str,
    html_content: &'a str,
}

#[derive(Serialize)]
struct EmailContact<'a> {
    email: &'a str,
}

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use claim::{assert_err, assert_ok};
    use fake::{
        Fake,
        faker::{
            internet::en::SafeEmail,
            lorem::en::{Paragraph, Sentence},
        },
    };
    use secrecy::SecretString;
    use wiremock::{
        Mock, MockServer, Request, ResponseTemplate,
        matchers::{any, header, header_exists, method, path},
    };

    use crate::{domain::SubscriberEmail, email_client::EmailClient};

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let parsed_body: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);

            if let Ok(body) = parsed_body {
                return body.get("Sender").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlContent").is_some();
            }
            false
        }
    }

    /// Generate a random email subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }
    /// Generate a random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }
    /// Generate a random subscriber email
    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }
    /// Get a test instance of `EmailClient`.
    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url,
            email(),
            Duration::from_millis(200),
            SecretString::from("123"),
        )
    }

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists("api-key"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            // Use our custom matcher!
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = email_client
            .send_email(email(), &subject(), &content())
            .await;
    }

    #[tokio::test]
    async fn send_email_success_if_server_returns_200() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_server_returns_500() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_timeout_if_server_takes_too_long() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(300)))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        assert_err!(outcome);
    }
}
