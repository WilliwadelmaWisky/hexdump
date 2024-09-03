
const BYTES_IN_LINE: usize = 16;
const WHITESPACE_ASCII: u8 = 32;
const NOT_PRINTABLE_ASCII: u8 = 46;

/// Checks if the byte is printable ascii character
fn is_printable_ascii(c: u8) -> bool {
    return c >= 32 && c <= 127;
}

/// Print the bytes in hexadecimal and show ASCII preview
fn hexdump(bytes: Vec<u8>) {
    let mut preview_buffer: [u8; BYTES_IN_LINE] = [WHITESPACE_ASCII; BYTES_IN_LINE];
    for (i, c) in bytes.iter().enumerate() {
        if i % BYTES_IN_LINE == 0 {
            print!("{:08X?}  ", i);
        }

        preview_buffer[i % BYTES_IN_LINE] = if is_printable_ascii(*c) { *c } else { NOT_PRINTABLE_ASCII };    
        print!("{:02X?} ", c);

        if i == bytes.len() - 1 && (i + 1) % BYTES_IN_LINE != 0 {
            let fill_amount = BYTES_IN_LINE - ((i + 1) % BYTES_IN_LINE);
            for _ in 0..fill_amount {
                print!("   ");
            }
        }

        if (i + 1) % BYTES_IN_LINE == 0 || i == bytes.len() - 1 {
            let s = String::from_utf8(preview_buffer.to_vec()).unwrap();
            print!(" | {} |", s);
            print!("\n");

            for i in 0..preview_buffer.len() {
                preview_buffer[i] = WHITESPACE_ASCII;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.get(0) {
        Some(s) => {
            let input_path = std::path::PathBuf::from(s);
            match std::fs::canonicalize(input_path) {
                Ok(absolute_path) => {
                    if absolute_path.is_dir() {
                        println!("FAILED: Cannot read a directory!");
                        return;
                    }
                    
                    let byte_array = std::fs::read(absolute_path).unwrap();
                    hexdump(byte_array);
                },
                Err(_) => {
                    println!("FAILED: Invalid filepath was given!");
                }
            }
        },
        None => {
            println!("FAILED: No path to a file was given!");
        }
    }
    
}
