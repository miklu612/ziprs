use super::crc32::crc32;


pub const LOCAL_FILE_ENTRY_IDENTIFIER: u64 = 0x04034b50;
pub const LOCAL_FILE_ENTRY_MIN_SIZE: usize = 30;
pub const LOCAL_FILE_ENTRY_MIN_VERSION: u16 = 60;
pub const LOCAL_FILE_ENTRY_GENERAL_BIT: u16 = 0;
pub const LOCAL_FILE_ENTRY_COMPRESSION_METHOD: u16 = 0;

pub struct File {
    name: String,
    content: Vec<u8>
}


impl File {

    #[allow(unused)]
    pub fn new() -> Self {
        File {
            name: String::new(),
            content: Vec::new()
        }
    }

    #[allow(unused)]
    pub fn from(name: &str, content: Vec<u8>) -> Self {
        File {
            name: name.to_string(),
            content: content
        }
    }

    pub fn calculate_central_directory_size(self: &Self) -> usize {
        46 + self.get_name_size()
    }
    pub fn calculate_size(self: &Self) -> usize {
        LOCAL_FILE_ENTRY_MIN_SIZE + self.get_name_size() + self.get_content_size()
    }
    pub fn get_crc32(self: &Self) -> u32 {
        crc32(&self.content)
    }
    pub fn get_content_size(self: &Self) -> usize {
        self.content.len()
    }
    pub fn get_name_size(self: &Self) -> usize {
        self.name.as_bytes().len()
    }
    pub fn compile_name(self: &Self) -> Vec<u8> {
        let mut output = Vec::new();
        output.reserve(self.get_name_size());
        for byte in self.name.as_bytes() {
            output.push(*byte);
        }
        return output;
    }
    pub fn compile_local_file_entry(self: &Self) -> Vec<u8> {

        let mut output: Vec<u8> = Vec::new();
        output.reserve(self.calculate_size());

        // File header stuff
        output.push(((LOCAL_FILE_ENTRY_IDENTIFIER >>  0) & 0xFF) as u8);
        output.push(((LOCAL_FILE_ENTRY_IDENTIFIER >>  8) & 0xFF) as u8);
        output.push(((LOCAL_FILE_ENTRY_IDENTIFIER >> 16) & 0xFF) as u8);
        output.push(((LOCAL_FILE_ENTRY_IDENTIFIER >> 24) & 0xFF) as u8);

        // Version
        output.push(((LOCAL_FILE_ENTRY_MIN_VERSION >> 0) & 0xFF) as u8);
        output.push(((LOCAL_FILE_ENTRY_MIN_VERSION >> 8) & 0xFF) as u8);

        // General purpose flag (not used)
        output.push(((LOCAL_FILE_ENTRY_GENERAL_BIT >> 0) & 0xFF) as u8);
        output.push(((LOCAL_FILE_ENTRY_GENERAL_BIT >> 8) & 0xFF) as u8);

        // Compression method
        output.push(((LOCAL_FILE_ENTRY_COMPRESSION_METHOD >> 0) & 0xFF) as u8);
        output.push(((LOCAL_FILE_ENTRY_COMPRESSION_METHOD >> 8) & 0xFF) as u8);
        
        // Last modification time
        output.push(((0 >> 0) & 0xFF) as u8);
        output.push(((0 >> 8) & 0xFF) as u8);

        // Last modification date
        output.push(((0 >> 0) & 0xFF) as u8);
        output.push(((0 >> 8) & 0xFF) as u8);

        // CRC-32
        let crc32_checksum = self.get_crc32();
        output.push(((crc32_checksum >>  0) & 0xFF) as u8);
        output.push(((crc32_checksum >>  8) & 0xFF) as u8);
        output.push(((crc32_checksum >> 16) & 0xFF) as u8);
        output.push(((crc32_checksum >> 24) & 0xFF) as u8);

        // Compressed length
        output.push(((self.content.len() >>  0) & 0xFF) as u8);
        output.push(((self.content.len() >>  8) & 0xFF) as u8);
        output.push(((self.content.len() >> 16) & 0xFF) as u8);
        output.push(((self.content.len() >> 24) & 0xFF) as u8);

        // Uncompressed length
        output.push(((self.content.len() >>  0) & 0xFF) as u8);
        output.push(((self.content.len() >>  8) & 0xFF) as u8);
        output.push(((self.content.len() >> 16) & 0xFF) as u8);
        output.push(((self.content.len() >> 24) & 0xFF) as u8);

        // File name length
        output.push(((self.name.as_bytes().len() >>  0) & 0xFF) as u8);
        output.push(((self.name.as_bytes().len() >>  8) & 0xFF) as u8);

        // Extra field length
        output.push(((0 >> 0) & 0xFF) as u8);
        output.push(((0 >> 8) & 0xFF) as u8);

        for byte in self.name.as_bytes() {
            output.push(*byte);
        }

        for byte in &self.content {
            output.push(*byte);
        }

        return output;
    }
}