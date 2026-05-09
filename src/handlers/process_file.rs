use std::fs::File;
use std::io::{BufWriter, Read, Result, Write};

use super::process_chunk::process_chunk;

pub fn process_file(file_to_read: &str, file_to_dump: &str) -> Result<()> {
    let mut in_file = File::open(file_to_read)?;
    let out_file = File::create(file_to_dump)?;

    let mut buffer = vec![0u8; 16];
    let mut offset: u64 = 0;
    let mut out_buffer = BufWriter::new(out_file);

    loop {
        let bytes_read = in_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let chunk = &buffer[..bytes_read];
        process_chunk(&mut out_buffer, chunk, offset)?;
        offset += bytes_read as u64;
    }

    out_buffer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_process_file() {
        let test_file_path = "test_input.bin";
        let dump_file_path = "test_dump.txt";

        let test_data = vec![
            0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
            0xAA, 0xBB, 0xCC, 0xDD,
        ];

        let mut test_file = File::create(test_file_path).unwrap();
        test_file.write_all(&test_data).unwrap();

        process_file(test_file_path, dump_file_path).unwrap();

        let expected_output =
            "00000000\tDE AD BE EF 00 11 22 33 44 55 66 77 88 99 AA BB \n00000010\tCC DD \n";
        let actual_output = fs::read_to_string(dump_file_path).unwrap();
        assert_eq!(actual_output, expected_output);

        fs::remove_file(test_file_path).unwrap();
        fs::remove_file(dump_file_path).unwrap();
    }
}
