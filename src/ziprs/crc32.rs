
// Translated from https://lxp32.github.io/docs/a-simple-example-crc32-calculation/
pub fn crc32(content: &Vec<u8>) -> u32 {

    let mut crc: u32 = 0xFFFFFFFF;

    for i in 0..content.len()  {
        let mut byte = content[i];
        for _ in 0..8 {
            let b = ((byte as u32)^crc)&1;
            crc = crc >> 1;
            if b != 0 {
                crc = crc ^ 0xEDB88320;
            }
            byte = byte >> 1;
        }
    }
    
    !crc
}
