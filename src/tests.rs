#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn hexdump_one() {
        let data: [u8; 1] = [0x90];
        assert_eq!(
            dump_line(&data, 0),
            format!(
                "00000000: 90                                               {NON_PRINTABLE_CHAR}"
            )
        );
    }
    #[test]
    fn hexdump_a() {
        let data: [u8; 1] = ['A' as u8];
        assert_eq!(
            dump_line(&data, 0),
            format!("00000000: 41                                               A")
        );
    }
    #[test]
    fn hexdump_one_zero() {
        let data: [u8; 1] = [0u8];
        assert_eq!(
            dump_line(&data, 0),
            format!(
                "00000000: 00                                               {NON_PRINTABLE_CHAR}"
            )
        );
    }
    #[test]
    fn hexdump_four() {
        let data: &[u8] = "paix".as_bytes();
        assert_eq!(
            dump_line(&data, 0),
            format!("00000000: 70 61 69 78                                      paix")
        );
    }
    #[test]
    fn hexdump_five() {
        let data: &[u8] = "paixx".as_bytes();
        assert_eq!(
            dump_line(&data, 0),
            format!(
                "00000000: 70 61 69 78{SEPARATOR_DWORD}78                                   paixx"
            )
        );
    }
    #[test]
    fn hexdump_eight() {
        let data: &[u8] = "fernando".as_bytes();
        assert_eq!(
            dump_line(data, 0),
            format!(
                "00000000: 66 65 72 6E{SEPARATOR_DWORD}61 6E 64 6F                          fernando"
            )
        );
    }
    #[test]
    fn hexdump_sixteen() {
        let data: &[u8] = "1234567812345678".as_bytes();
        assert_eq!(
            dump_line(data, 0),
            format!(
                "00000000: 31 32 33 34{SEPARATOR_DWORD}35 36 37 38{SEPARATOR_DWORD}31 32 33 34{SEPARATOR_DWORD}35 36 37 38  1234567812345678"
            )
        );
    }
}
