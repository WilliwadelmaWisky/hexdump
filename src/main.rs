
const BYTES_IN_LINE: usize = 16;
const DEFAULT_PREVIEW_CHARACTER: u8 = 32;
const NOT_PRINTABLE_CHARACTER: u8 = 46;

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
                    let mut preview_buffer: [u8; BYTES_IN_LINE] = [DEFAULT_PREVIEW_CHARACTER; BYTES_IN_LINE];
                    for (index, value) in byte_array.iter().enumerate() {
                        preview_buffer[index % BYTES_IN_LINE] = if *value >= 32 && *value <= 127 { *value } else { NOT_PRINTABLE_CHARACTER };    
                        print!("{:02X?} ", value);
                        if index == byte_array.len() - 1 {
                            let fill_amount = BYTES_IN_LINE - ((index + 1) % BYTES_IN_LINE);
                            for _ in 0..fill_amount {
                                print!("   ");
                            }
                        }
                        if (index + 1) % BYTES_IN_LINE == 0 || index == byte_array.len() - 1 {
                            let s = String::from_utf8(preview_buffer.to_vec()).unwrap();
                            print!("| {} |", s);
                            print!("\n");
        
                            for i in 0..preview_buffer.len() {
                                preview_buffer[i] = DEFAULT_PREVIEW_CHARACTER;
                            }
                        }
                    }
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
