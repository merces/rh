use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[cfg(test)]
mod tests;

const CHUNK_SIZE: usize = 4096; // 4KB
const SEPARATOR_DWORD: char = ' ';
const NON_PRINTABLE_CHAR: char = ' ';

fn dump_line(data: &[u8], offset: usize) -> String {
    assert!(data.len() <= 16);

    let mut line = String::with_capacity(76);
    let mut ascii = String::with_capacity(16);

    line.push_str(&format!("{offset:08X}:"));

    for (i, byte) in data.iter().enumerate() {
        if i > 0 && i % 4 == 0 {
            line.push(SEPARATOR_DWORD);
        } else {
            line += " ";
        }

        line += &format!("{:02X}", byte);

        // handle ASCII
        let c = *byte as char;
        if c.is_ascii_graphic() {
            ascii.push(c);
        } else {
            ascii.push(NON_PRINTABLE_CHAR);
        }
    }

    line = format!("{:59}", line);
    line.push_str(&ascii);
    line
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("I need a file to work with. 🥹");
        exit(1);
    }

    let path = &args[1];
    let mut file = File::open(path).expect("File not found");

    let mut ofs = 0;
    loop {
        let mut buffer = [0u8; CHUNK_SIZE];

        let bytes_read = match file.read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading file: {e}");
                return;
            }
        };

        if bytes_read == 0 {
            break;
        }

        let mut pos = 0;
        while pos < bytes_read {
            let end = (pos + 16).min(bytes_read); // it can't exceed `bytes_read`
            println!("{}", dump_line(&buffer[pos..end], ofs + pos));
            pos += 16;
        }

        ofs += bytes_read;
    }

    // useful to quickly know the file size :)
    println!("{ofs:08x}:");
}
