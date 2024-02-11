# Zip.rs

Zip.rs is a simple library for writing uncompressed zip files. This project 
has zero dependencies.

## Example

```rust
// examples/hello_world.rs
use std::io::Write;
extern crate ziprs;

fn write_file(content: Vec<u8>) {
    let mut file = std::fs::File::create("output.zip").expect("Couldn't create file");
    file.write_all(&content as &[u8]).expect("Couldn't write file");
} 

fn main() {
    let mut writer = ziprs::Writer::new();
    writer.add_file(ziprs::File::from("file-1.txt", "The first file".into()));
    writer.add_file(ziprs::File::from("file-2.txt", "The second file".into()));
    write_file(writer.compile())
}
```