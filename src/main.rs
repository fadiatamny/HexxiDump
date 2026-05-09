mod handlers;

use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::SystemTime;

use handlers::process_file::process_file;

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

    if let Err(e) = process_file(program_to_read, &dump_path) {
        eprintln!("An error occurred: {}", e);
        process::exit(1);
    }
}
