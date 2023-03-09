use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct BinaryAttachment {
    content_type: String,
    file_name: String,
    binary_data: Vec<u8>,
}

impl BinaryAttachment {
    pub fn new() -> Self {
        BinaryAttachment::default()
    }

    pub fn add_content_type(&mut self, content_type: &str) -> &mut Self {
        self.content_type = content_type.to_owned();

        self
    }

    pub fn add_file_name(&mut self, file_name: &str) -> &mut Self {
        self.file_name = file_name.to_owned();

        self
    }

    pub fn add_binary_data(&mut self, binary_data: Vec<u8>) -> &mut Self {
        self.binary_data = binary_data;

        self
    }

    pub fn content_type(&self) -> &String {
        &self.content_type
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn binary_data(&self) -> &[u8] {
        &self.binary_data
    }
}
