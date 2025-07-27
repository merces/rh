use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

const CHUNK_SIZE: usize = 4096; // 4KB
const SEPARATOR_DWORD: char = ' ';
const NON_PRINTABLE_CHAR: char = ' ';

fn dump_line(data: &[u8], offset: usize) -> Result<String, &'static str> {
    let data_len = data.len();

    if data_len > 16 {
        return Err("Warning: received more than 16 bytes of data. Only 16 bytes will be taken.");
    }

    let mut line = String::with_capacity(76);
    let mut ascii = String::with_capacity(16);

    line.push_str(&format!("{offset:08X}:"));

    for (i, byte) in data.iter().take(16).enumerate() {
        if i > 0 && i % 4 == 0 {
            line.push(SEPARATOR_DWORD);
        } else {
            line += " ";
        }

        line += &format!("{:02X}", byte);

        // handle ASCII
        let c = *byte as char;
        if c.is_ascii_graphic() || c == ' ' {
            ascii.push(c);
        } else {
            ascii.push(NON_PRINTABLE_CHAR);
        }
    }

    line = format!("{:59}", line);
    line.push_str(&ascii);
    Ok(line)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("I need a file to work with. 🥹");
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
            match dump_line(&buffer[pos..end], ofs + pos) {
                Ok(line) => println!("{line}"),
                Err(err) => eprintln!("{err}"),
            };
            pos += 16;
        }

        ofs += bytes_read;
    }

    // useful to quickly know the file size :)
    println!("{ofs:08x}:");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hexdump_one() {
        let data: [u8; 1] = [0x90];
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 90                                               {NON_PRINTABLE_CHAR}"
            ))
        );
    }
    #[test]
    fn hexdump_a() {
        let data: [u8; 1] = ['A' as u8];
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 41                                               A"
            ))
        );
    }
    #[test]
    fn hexdump_one_zero() {
        let data: [u8; 1] = [0u8];
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 00                                               {NON_PRINTABLE_CHAR}"
            ))
        );
    }
    #[test]
    fn hexdump_four() {
        let data: &[u8] = "paix".as_bytes();
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 70 61 69 78                                      paix"
            ))
        );
    }
    #[test]
    fn hexdump_five() {
        let data: &[u8] = "paixx".as_bytes();
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 70 61 69 78{SEPARATOR_DWORD}78                                   paixx"
            ))
        );
    }
    #[test]
    fn hexdump_eight() {
        let data: &[u8] = "fernando".as_bytes();
        assert_eq!(
            dump_line(data, 0),
            Ok(format!(
                "00000000: 66 65 72 6E{SEPARATOR_DWORD}61 6E 64 6F                          fernando"
            ))
        );
    }
    #[test]
    fn hexdump_sixteen() {
        let data: &[u8] = "1234567812345678".as_bytes();
        assert_eq!(
            dump_line(data, 0),
            Ok(format!(
                "00000000: 31 32 33 34{SEPARATOR_DWORD}35 36 37 38{SEPARATOR_DWORD}31 32 33 34{SEPARATOR_DWORD}35 36 37 38  1234567812345678"
            ))
        );
    }
}
