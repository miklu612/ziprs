use std::io::Write;


mod ziprs;

fn write_file(content: Vec<u8>) {
    let mut file = std::fs::File::create("test/output.zip").expect("Couldn't create file");
    file.write_all(&content as &[u8]).expect("Couldn't write file");
} 

fn test_crc32() {
    let test_input = "Hello, World!".as_bytes().to_vec();
    let expected_output: u32 = 0xEC4AC3D0;
    assert!(expected_output == ziprs::crc32::crc32(&test_input));
}

fn main() {


     
    let mut writer: ziprs::Writer = ziprs::Writer::new();
    writer.add_file(ziprs::File::from("hello-1.txt", "Wow".to_string().into_bytes()));
    writer.add_file(ziprs::File::from("hello-2.txt", "Wowie".to_string().into_bytes()));
    write_file(writer.compile());
    
}
