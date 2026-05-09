use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::time::SystemTime;

fn process_chunk(out_file: &mut File, chunk: &[u8], offset: usize) -> Result<()> {
    write!(out_file, "{:08X}\t", offset)?;
    for byte in chunk {
        out_file.write(format!("{:02X} ", byte).as_bytes())?;
    }
    writeln!(out_file)?;

    Ok(())
}

fn process_file_in_chunks(file_to_read: &str, file_to_dump: &str) -> Result<()> {
    println!("Reading From: {file_to_read}");

    let file = fs::File::open(file_to_read);

    if let Err(err) = file {
        println!("Error opening file: {}", err);
        return Err(err);
    }

    let out_file = fs::File::create(file_to_dump);

    if let Err(err) = out_file {
        println!("Error creating output file: {}", err);
        return Err(err);
    }

    let mut opened_file = file.unwrap();
    let mut out_opened_file = out_file.unwrap();

    let mut buffer = vec![0u8; 16];
    let mut offset = 0;

    loop {
        let bytes_read = match opened_file.read(&mut buffer) {
            Ok(bytes) => bytes,
            Err(err) => {
                println!("Error reading file: {}", err);
                return Err(err);
            }
        };

        if bytes_read == 0 {
            break;
        }
        let chunk = &buffer[..bytes_read];

        if let Err(err) = process_chunk(&mut out_opened_file, chunk, offset) {
            println!("Error processing chunk: {}", err);
            return Err(err);
        }

        offset += bytes_read;
    }

    Ok(())
}

fn main() {
    fs::create_dir_all("./dumps").expect("Failed to create the dumps directory");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file_to_read>", args[0]);
        return;
    }

    let programm_to_read = &args[1];
    let dump_path = if args.len() == 3 {
        args[3].clone()
    } else {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!("./dumps/dump_{}.txt", timestamp)
    };

    if let Err(e) = process_file_in_chunks(programm_to_read, &dump_path) {
        println!("An error occurred: {}", e);
    }
}
