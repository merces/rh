use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

/// O tamanho do buffer para ler leitura do arquivo
const BUFFER_SIZE: usize = 4096; // 4KB

/// O caractere que vai separar cada _double word_
/// (quatro bytes) na visualização hexadecimal.
const SEPARATOR_DWORD: char = ' ';

/// Se um byte não faz parte da faixa imprimível
/// da tabela ASCII, o caractere que será impresso
/// no lugar dele.
const NON_PRINTABLE_CHAR: char = ' ';

/// Recebe 16 bytes de dados e um offset e os retorna
/// formatados no estilo de um visualizador hexadecimal
/// de linha de comando, tipo `hexdump`, `xxd`, etc.
///
/// # Argumentos
///
/// * `data` - Uma referência para um slide de bytes, que deve
///   conter até 16 bytes para serem formatados.
///
/// * `offset` - o valor do offset que será formatado junto
///   aos bytes.
///
/// # Retorna
///
/// Uma `String` contendo o offset, os bytes formatados e
/// o equivalente ASCII dos bytes, se houver. Ou uma string de erro.
///
/// # Exemplo
///
/// Assumindo que `SEPARATOR_DWORD` é `' '`:
/// ```
/// let data: &[u8] = "qualquercoisa".as_bytes();
/// assert_eq!(
///         dump_line(data, 0),
///         Ok(format!("00000000: 71 75 61 6C 71 75 65 72 63 6F 69 73 61           qualquercoisa"))
///     );
/// ```
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

        line += &format!("{byte:02X}");

        // handle ASCII
        let c = *byte as char;
        if c.is_ascii_graphic() || c == ' ' {
            ascii.push(c);
        } else {
            ascii.push(NON_PRINTABLE_CHAR);
        }
    }

    line = format!("{line:59}{ascii}");
    Ok(line)
}

/// Usa um buffer de `BUFFER_SIZE` bytes
/// para ler chunks de 16 bytes dele e enviar
/// esses slices para a função dump_line()
///
/// # Argumentos
///
/// * `reader` - um ponteiro inteligente `Box<T>` para um tipo que implemente
///   a trait `Read` (normalmente `File` ou `Stdin`).
///
/// # Retorna
///
/// O número de bytes lidos/"dumpados" ou std::io::Error.
fn dump_file<R: Read>(reader: R) -> Result<usize, std::io::Error> {
    let mut buf_reader = BufReader::with_capacity(BUFFER_SIZE, reader);
    let mut chunk: [u8; 16] = [0u8; 16];
    let mut ofs = 0;

    loop {
        let bytes_read = match buf_reader.read(&mut chunk) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };

        if bytes_read == 0 {
            break;
        }

        match dump_line(&chunk[..bytes_read], ofs) {
            Ok(line) => println!("{line}"),
            Err(e) => println!("{e}"),
        };

        ofs += bytes_read;
    }

    Ok(ofs)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // reader será um `Box<T>``, ou seja, um ponteiro inteligente (alocado na heap).
    // Ao declará-la como `Box<dyn Read>``, digo que é um ponteiro para qualquer tipo
    // que implemente a trait Read. Dessa forma, dump_file() pode receber tanto
    // um File, retornado por File::open() em caso de sucesso, quanto Stdin,
    // retornado por std::io::stdin().
    let reader: Box<dyn Read> = if args.len() > 1 {
        Box::new(File::open(&args[1])?)
    } else {
        Box::new(std::io::stdin())
    };

    // dump_file() retorna o número de bytes "dumpados", que imprimo na tela.
    // É uma maneira fácil de visualizar o tamanho do arquivo também.
    println!("{:08x}:", dump_file(reader)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn dump_line_one() {
        let data: [u8; 1] = [0x90];
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 90                                               {NON_PRINTABLE_CHAR}"
            ))
        );
    }
    #[test]
    fn dump_line_a() {
        let data: [u8; 1] = ['A' as u8];
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 41                                               A"
            ))
        );
    }
    #[test]
    fn dump_line_one_zero() {
        let data: [u8; 1] = [0u8];
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 00                                               {NON_PRINTABLE_CHAR}"
            ))
        );
    }
    #[test]
    fn dump_line_four() {
        let data: &[u8] = "paix".as_bytes();
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 70 61 69 78                                      paix"
            ))
        );
    }
    #[test]
    fn dump_line_five() {
        let data: &[u8] = "paixx".as_bytes();
        assert_eq!(
            dump_line(&data, 0),
            Ok(format!(
                "00000000: 70 61 69 78{SEPARATOR_DWORD}78                                   paixx"
            ))
        );
    }
    #[test]
    fn dump_line_eight() {
        let data: &[u8] = "fernando".as_bytes();
        assert_eq!(
            dump_line(data, 0),
            Ok(format!(
                "00000000: 66 65 72 6E{SEPARATOR_DWORD}61 6E 64 6F                          fernando"
            ))
        );
    }
    #[test]
    fn dump_line_sixteen() {
        let data: &[u8] = "1234567812345678".as_bytes();
        assert_eq!(
            dump_line(data, 0),
            Ok(format!(
                "00000000: 31 32 33 34{SEPARATOR_DWORD}35 36 37 38{SEPARATOR_DWORD}31 32 33 34{SEPARATOR_DWORD}35 36 37 38  1234567812345678"
            ))
        );
    }
    #[test]
    fn dump_file_ls() {
        let file_path = "/bin/ls";
        let file = File::open(file_path).unwrap();
        let metadata = fs::metadata(file_path).unwrap();
        let file_size = dump_file(file).unwrap();
        assert_eq!(metadata.len(), file_size as u64);
    }
}
