#[allow(unused_imports)]
use esp_idf_sys; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal;

#[derive(Debug, PartialEq)]
enum WriteMethod {
    Direct,
    Write,
    WriteU8,
}

use std::{str::FromStr};

impl FromStr for WriteMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<WriteMethod, Self::Err> {
        match &(input.to_ascii_lowercase())[..] {
            "direct"    => Ok(WriteMethod::Direct),
            "write"     => Ok(WriteMethod::Write),
            "writeu8"   => Ok(WriteMethod::WriteU8),
            _           => Err(()),
        }
    }
}


use byteorder::WriteBytesExt;
use anyhow::*;
use std::io::Write;

/// writes 256*4=1024 bytes to allocated vector, so minimum kb_blocks should probably be 2
fn test_memory_allocation(kb_blocks:usize, step:usize, method:WriteMethod) -> Result<Vec<u8>, Error> {
    assert!(kb_blocks >= 2, "kb_blocks needs to be at least 2 since will try to write 256*4 bytes");


    const KILOBYTE: usize = 1024;

    let mut my_vec: Vec<u8> = vec![];

    for i in (step..=kb_blocks).step_by(step) {
        let size = i * KILOBYTE;
        my_vec = Vec::with_capacity(size);
        println!("{}: allocated Vec<u8> of size: {}", i, size);

        // simulate writing 8-bit Bitmap Palette with 256 entries of 4-bytes each

        match method {
            WriteMethod::Direct => {

                for val in 0u8..255u8 {
                    my_vec.push(val);
                    my_vec.push(val);
                    my_vec.push(val);
                    my_vec.push(0);
                }
            },
            WriteMethod::Write => {

                // turn Vector into Writable via Cursor
                let mut cursored  = std::io::Cursor::new(&mut my_vec);
                let writable_buffer: &mut dyn Write = &mut cursored;
                
                for val in 0u8..=255u8 {
                    writable_buffer.write_all(&[val, val, val, 0] )?;
                }

            },
            WriteMethod::WriteU8 => {

                // turn Vector into Writable via Cursor
                let mut cursored  = std::io::Cursor::new(&mut my_vec);
                let writable_buffer: &mut dyn Write = &mut cursored;

                for val in 0u8..255u8 {
                    writable_buffer.write_u8(val)?;
                    writable_buffer.write_u8(val)?;
                    writable_buffer.write_u8(val)?;
                    writable_buffer.write_u8(0)?;
                }

            }
        }


    }

    println!("Allocated {:?} KB blocks in step of {:?} and wrote using method '{:?}'", &kb_blocks, &step, &method);

    Ok(my_vec)
}

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    let choice = unsafe { esp_idf_sys::esp_random() % 3 };

    println!("Running memory test #{}", choice);

    // run memory test in random order
    match choice {
        0 => {
            println!("Running 2 KB memory test with WriteMethod::Direct...");
            let _test_vec1 = test_memory_allocation(2, 2, WriteMethod::Direct).unwrap();
        },
        1 => {
            println!("Running 2 KB memory test with WriteMethod::Write...");
            let _test_vec3 = test_memory_allocation(2, 2, WriteMethod::Write).unwrap();
        },
        2 => {
            println!("Running 2 KB memory test with WriteMethod::WriteU8...");
            let _test_vec2 = test_memory_allocation(2, 2, WriteMethod::WriteU8).unwrap();
        }
        _ => { 
            panic!("Enexpected choice: {}", choice);
        }
    }

    println!("Memory test #{} PASSED !", choice);

    for i in (1..=10).rev() {
        println!("Restarting in {} seconds...", i);
        unsafe { esp_idf_sys::vTaskDelay(1000 / esp_idf_hal::delay::portTICK_PERIOD_MS); }
    }
    println!("Restarting now.\n");
    unsafe {esp_idf_sys::esp_restart(); }

}
