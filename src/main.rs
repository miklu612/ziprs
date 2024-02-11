use std::io::Write;


mod ziprs;

fn write_file(content: Vec<u8>) {
    let mut file = std::fs::File::create("test/output.zip").expect("Couldn't create file");
    file.write_all(&content as &[u8]).expect("Couldn't write file");
} 

fn main() {
    let mut writer: ziprs::Writer = ziprs::Writer::new();
    writer.add_file(ziprs::File::from("hello-1.txt", "Wow".to_string().into_bytes()));
    writer.add_file(ziprs::File::from("hello-2.txt", "Wowie".to_string().into_bytes()));
    write_file(writer.compile());
}
