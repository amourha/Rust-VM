mod vm;
mod instruction;
mod verify;

use std::fs;

use verify::SimpleKeyCheck;

fn main() {
    let key: [u8; 16] = [0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x7f, 0x7f,
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x37, 0x13];

    let data = fs::read("example/check").expect("Unable to read file");
    

    let mut checker = SimpleKeyCheck::new(&key);
    checker.set_program(&data);

    if checker.run() {
        println!("Key is correct");
    }
    else {
        println!("Key is incorrect");
    }
}
