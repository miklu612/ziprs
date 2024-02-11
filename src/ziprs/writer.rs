use super::File;
use super::file::LOCAL_FILE_ENTRY_MIN_VERSION;

const CENTRAL_FILE_HEADER_IDENTIFIER: u32 = 0x02014b50;
const CENTRAL_FILE_HEADER_VERSION: u16 = LOCAL_FILE_ENTRY_MIN_VERSION;
const CENTRAL_FILE_HEADER_GENERAL_BIT: u16 = 0;
const CENTRAL_FILE_HEADER_COMPRESSION_METHOD: u16 = 0;
const CENTRAL_FILE_HEADER_DISK_NUMBER: u16 = 0;
const CENTRAL_FILE_HEADER_INTERNAL_FILE_ATTRIBUTE: u16 = 0;
const CENTRAL_FILE_HEADER_EXTERNAL_FILE_ATTRIBUTE: u32 = 0;

const END_OF_CENTRAL_DIRECTORY_IDENTIFIER: u32 = 0x06054b50;
const END_OF_CENTRAL_DIRECTORY_DISK_NUMBER: u16 = CENTRAL_FILE_HEADER_DISK_NUMBER;
const END_OF_CENTRAL_DIRECTORY_START_DISK_NUMBER: u16 = CENTRAL_FILE_HEADER_DISK_NUMBER;


pub struct Writer {
    files: Vec<File>
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            files: Vec::new()
        }
    }
    pub fn add_file(self: &mut Self, file: File) {
        self.files.push(file);
    }
    pub fn create_central_file_headers(self: &Self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        let mut offset: usize = 0;
        for file in &self.files {
            // Version
            output.push(((CENTRAL_FILE_HEADER_IDENTIFIER >>  0) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_IDENTIFIER >>  8) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_IDENTIFIER >> 16) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_IDENTIFIER >> 24) & 0xFF) as u8);

            // Version
            output.push(((CENTRAL_FILE_HEADER_VERSION >>  0) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_VERSION >>  8) & 0xFF) as u8);

            // Min Version
            output.push(((CENTRAL_FILE_HEADER_VERSION >>  0) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_VERSION >>  8) & 0xFF) as u8);

            // General bit
            output.push(((CENTRAL_FILE_HEADER_GENERAL_BIT >>  0) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_GENERAL_BIT >>  8) & 0xFF) as u8);

            // Compression Method
            output.push(((CENTRAL_FILE_HEADER_COMPRESSION_METHOD >>  0) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_COMPRESSION_METHOD >>  8) & 0xFF) as u8);

            // Modification time
            output.push(((0 >>  0) & 0xFF) as u8);
            output.push(((0 >>  8) & 0xFF) as u8);

            // Modification date
            output.push(((0 >>  0) & 0xFF) as u8);
            output.push(((0 >>  8) & 0xFF) as u8);

            // Crc32 checksum
            let crc32_checksum = file.get_crc32();
            output.push(((crc32_checksum >>  0) & 0xFF) as u8);
            output.push(((crc32_checksum >>  8) & 0xFF) as u8);
            output.push(((crc32_checksum >> 16) & 0xFF) as u8);
            output.push(((crc32_checksum >> 24) & 0xFF) as u8);

            // Compressed file size 
            output.push(((file.get_content_size() >>  0) & 0xFF) as u8);
            output.push(((file.get_content_size() >>  8) & 0xFF) as u8);
            output.push(((file.get_content_size() >> 16) & 0xFF) as u8);
            output.push(((file.get_content_size() >> 24) & 0xFF) as u8);

            // Uncompressed file size 
            output.push(((file.get_content_size() >>  0) & 0xFF) as u8);
            output.push(((file.get_content_size() >>  8) & 0xFF) as u8);
            output.push(((file.get_content_size() >> 16) & 0xFF) as u8);
            output.push(((file.get_content_size() >> 24) & 0xFF) as u8);

            // Name size 
            output.push(((file.get_name_size() >>  0) & 0xFF) as u8);
            output.push(((file.get_name_size() >>  8) & 0xFF) as u8);

            // extra field 
            output.push(((0 >>  0) & 0xFF) as u8);
            output.push(((0 >>  8) & 0xFF) as u8);

            // Comment
            output.push(((0 >>  0) & 0xFF) as u8);
            output.push(((0 >>  8) & 0xFF) as u8);
            
            // Disk
            output.push(((CENTRAL_FILE_HEADER_DISK_NUMBER >>  0) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_DISK_NUMBER >>  8) & 0xFF) as u8);

            // Internal file attribute
            output.push(((CENTRAL_FILE_HEADER_INTERNAL_FILE_ATTRIBUTE >>  0) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_INTERNAL_FILE_ATTRIBUTE >>  8) & 0xFF) as u8);

            // External file attribute 
            output.push(((CENTRAL_FILE_HEADER_EXTERNAL_FILE_ATTRIBUTE >>  0) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_EXTERNAL_FILE_ATTRIBUTE >>  8) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_EXTERNAL_FILE_ATTRIBUTE >> 16) & 0xFF) as u8);
            output.push(((CENTRAL_FILE_HEADER_EXTERNAL_FILE_ATTRIBUTE >> 24) & 0xFF) as u8);

            // offset 
            output.push(((offset >>  0) & 0xFF) as u8);
            output.push(((offset >>  8) & 0xFF) as u8);
            output.push(((offset >> 16) & 0xFF) as u8);
            output.push(((offset >> 24) & 0xFF) as u8);
            offset += file.calculate_size();

            output.extend(file.compile_name());

        }

        return output;
    }
    pub fn create_end_of_central_directory_header(self: &Self) -> Vec<u8> {

        let mut start_offset: usize = 0;
        let mut central_directory_size: usize = 0;

        for file in &self.files {
            start_offset += file.calculate_size();
            central_directory_size += file.calculate_central_directory_size();

        }


        
        let mut output = Vec::new();
        output.reserve(20);

        // Identifier
        output.push(((END_OF_CENTRAL_DIRECTORY_IDENTIFIER >>  0) & 0xFF) as u8);
        output.push(((END_OF_CENTRAL_DIRECTORY_IDENTIFIER >>  8) & 0xFF) as u8);
        output.push(((END_OF_CENTRAL_DIRECTORY_IDENTIFIER >> 16) & 0xFF) as u8);
        output.push(((END_OF_CENTRAL_DIRECTORY_IDENTIFIER >> 24) & 0xFF) as u8);

        // Disk number
        output.push(((END_OF_CENTRAL_DIRECTORY_DISK_NUMBER >>  0) & 0xFF) as u8);
        output.push(((END_OF_CENTRAL_DIRECTORY_DISK_NUMBER >>  8) & 0xFF) as u8);

        // Disk number start
        output.push(((END_OF_CENTRAL_DIRECTORY_START_DISK_NUMBER >>  0) & 0xFF) as u8);
        output.push(((END_OF_CENTRAL_DIRECTORY_START_DISK_NUMBER >>  8) & 0xFF) as u8);

        // Entry count this disk
        output.push(((self.files.len() >>  0) & 0xFF) as u8);
        output.push(((self.files.len() >>  8) & 0xFF) as u8);

        // Entry count overall
        output.push(((self.files.len() >>  0) & 0xFF) as u8);
        output.push(((self.files.len() >>  8) & 0xFF) as u8);

        // Central directory size
        output.push(((central_directory_size >>  0) & 0xFF) as u8);
        output.push(((central_directory_size >>  8) & 0xFF) as u8);
        output.push(((central_directory_size >> 16) & 0xFF) as u8);
        output.push(((central_directory_size >> 32) & 0xFF) as u8);

        // offset
        output.push(((start_offset >>  0) & 0xFF) as u8);
        output.push(((start_offset >>  8) & 0xFF) as u8);
        output.push(((start_offset >> 16) & 0xFF) as u8);
        output.push(((start_offset >> 24) & 0xFF) as u8);

        return output;
    }
    pub fn compile(self: &Self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        for file in &self.files {
            output.extend(file.compile_local_file_entry());
        }
        output.extend(self.create_central_file_headers());
        output.extend(self.create_end_of_central_directory_header());
        return output;
    }
}
