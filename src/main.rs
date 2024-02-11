use std::io::{Stdin, Write};
use file::File;
use crate::ziprs::file;


mod ziprs;

fn write_file(content: Vec<u8>) {
    let mut file = std::fs::File::create("output.zip").expect("Couldn't create file");
    file.write_all(&content as &[u8]).expect("Couldn't write file");
} 

#[allow(unused)]
fn test_crc32() {
    let test_input = "Hello, World!".as_bytes().to_vec();
    let expected_output: u32 = 0xEC4AC3D0;
    assert!(expected_output == ziprs::crc32::crc32(&test_input));
}

fn create_very_large_test_zip() -> Vec<u8> {
    let mut zip_writer = ziprs::Writer::new();
    let test_text = {
        "Hello, World!\n".to_owned() + 
        "This is the test file\n" +
        "Which test file is this?" +
        "This is the " 
    };
    for i in 0..1000 {
        let file_name = "hello-".to_owned() + &i.to_string() + ".txt";
        let file_content = test_text.clone() + &i.to_string() + "th file";
        zip_writer.add_file(File::from(&file_name, file_content.as_bytes().to_vec()));
    }
    zip_writer.compile()
}

fn remove_newlines(string: &String) -> String {
    let mut output = String::new();
    for character in string.chars().into_iter() {
        match character {
            '\n' => {},
            '\r' => {},
            _ => {
                output += &character.to_string();
            }
        }
    }
    output
}

fn main() {

    println!("Do you want to write a large file (100kb)? (Write 'yes')");

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer = remove_newlines(&buffer);
    if buffer == "yes" {
        write_file(create_very_large_test_zip());
    }
}
