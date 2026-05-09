use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Read, Result, Write};
use std::path::Path;
use std::process;
use std::time::SystemTime;

fn process_chunk<W: Write>(output: &mut W, chunk: &[u8], offset: u64) -> Result<()> {
    write!(output, "{:08X}\t", offset)?;
    for byte in chunk {
        write!(output, "{:02X} ", byte)?;
    }
    writeln!(output)?;

    Ok(())
}

fn process_file_in_chunks(file_to_read: &str, file_to_dump: &str) -> Result<()> {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_to_read> [output_dump_path]", args[0]);
        process::exit(1);
    }

    let program_to_read = &args[1];
    let dump_path = if args.len() == 3 {
        args[2].clone()
    } else {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!("./dumps/dump_{}.txt", timestamp)
    };

    if let Some(parent) = Path::new(&dump_path).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Failed to create directory {}: {}", parent.display(), e);
                process::exit(1);
            }
        }
    }

    if let Err(e) = process_file_in_chunks(program_to_read, &dump_path) {
        eprintln!("An error occurred: {}", e);
        process::exit(1);
    }
}
