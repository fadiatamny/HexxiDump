use std::env;
use std::fs;
use std::time::SystemTime;

fn main() {
    fs::create_dir_all("./dumps").expect("Failed to create the dumps directory");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file_to_read>", args[0]);
        return;
    }

    let program_name = &args[0];
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

    println!("[{program_name}] Reading From: {programm_to_read}");

    let content = fs::read(programm_to_read);

    if let Err(content) = content {
        println!("Error reading file: {}", content);
        return;
    }

    println!("[{program_name}] Dumping to {dump_path}");

    let write_operation = fs::write(dump_path, content.unwrap());

    if let Err(write_operation) = write_operation {
        println!("Error writing to file: {}", write_operation);
        return;
    }

    println!("[{program_name}] Dumping Completed Successfully!");
}
