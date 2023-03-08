use crate::BinaryAttachment;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct Envelope {
    to: Vec<(String, String)>,
    subject: String,
    html_body: Option<String>,
    attachments: Vec<BinaryAttachment>,
    text_body: Option<String>,
}

impl Envelope {
    pub fn new() -> Self {
        Envelope::default()
    }

    pub fn recipients(&self) -> &[(String, String)] {
        &self.to
    }

    pub fn subject(&self) -> &String {
        &self.subject
    }

    pub fn html_body(&self) -> Option<&String> {
        self.html_body.as_ref()
    }

    pub fn text_body(&self) -> Option<&String> {
        self.text_body.as_ref()
    }

    pub fn attachments(&self) -> &[BinaryAttachment] {
        self.attachments.as_ref()
    }

    pub fn add_recipient(&mut self, recipient: (&str, &str)) -> &mut Self {
        self.to
            .push((recipient.0.to_owned(), recipient.1.to_owned()));

        self
    }

    pub fn add_subject(&mut self, subject: &str) -> &mut Self {
        self.subject = subject.to_owned();

        self
    }

    pub fn add_html_body(&mut self, html_body: &str) -> &mut Self {
        self.html_body = Some(html_body.to_owned());

        self
    }

    pub fn add_text_body(&mut self, text_body: &str) -> &mut Self {
        self.text_body = Some(text_body.to_owned());

        self
    }

    pub fn add_attachment(&mut self, attachment: BinaryAttachment) -> &mut Self {
        self.attachments.push(attachment);

        self
    }
}
