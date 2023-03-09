use crate::MailerConfig;
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use xor_mailer_common::Envelope;

#[derive(Debug)]
pub struct Mailer<'x> {
    config: &'x MailerConfig,
    mail: Envelope,
}

impl<'x> Mailer<'x> {
    pub fn new(config: &'x MailerConfig) -> Self {
        Mailer {
            config,
            mail: Envelope::default(),
        }
    }

    pub fn add_config(&mut self, config: &'x MailerConfig) -> &mut Self {
        self.config = config;

        self
    }

    pub fn add_envelope(&mut self, envelope: Envelope) -> &mut Self {
        self.mail = envelope;

        self
    }

    /// If `html_body` field is some, prioritize it instead of `text_body` field which is a string.
    pub fn build(&self, from: (&str, &str)) -> MessageBuilder {
        let mut message = MessageBuilder::new();
        message
            .from((from.0.to_owned(), from.1.to_owned()))
            .to(self.mail.recipients().to_owned())
            .subject(self.mail.subject());

        if let Some(html_body) = self.mail.html_body() {
            message.html_body(html_body);
        } else {
            if let Some(text_body) = self.mail.text_body() {
                message.text_body(text_body);
            }
        }

        self.mail
            .attachments()
            .iter()
            .for_each(|binary_attachment| {
                message.binary_attachment(
                    binary_attachment.content_type(),
                    binary_attachment.file_name(),
                    binary_attachment.binary_data(),
                );
            });

        message
    }

    pub async fn send(&self) -> mail_send::Result<()> {
        let message = self.build(self.config.sender());

        // Connect to the SMTP submissions port, upgrade to TLS and
        // authenticate using the provided credentials.
        SmtpClientBuilder::new(self.config.smtp_uri(), self.config.smtp_port())
            .implicit_tls(false)
            .credentials(self.config.credentials())
            .connect()
            .await?
            .send(message)
            .await?;

        Ok(())
    }
}
