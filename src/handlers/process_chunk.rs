use std::io::Result;
use std::io::Write;

pub fn process_chunk<W: Write>(output: &mut W, chunk: &[u8], offset: u64) -> Result<()> {
    write!(output, "{:08X}\t", offset)?;
    for byte in chunk {
        write!(output, "{:02X} ", byte)?;
    }
    writeln!(output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_chunk() {
        let chunk = [0xDE, 0xAD, 0xBE, 0xEF];
        let offset = 0x1234;

        let mut buf: Vec<u8> = Vec::new();
        process_chunk(&mut buf, &chunk, offset).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, "00001234\tDE AD BE EF \n");
    }
}
