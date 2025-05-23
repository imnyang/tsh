use std::env;
use std::process;
use trash::delete;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: tsh <file_or_folder>...");
        process::exit(1);
    }

    for path in args {
        match delete(&path) {
            Ok(_) => println!("✅ {}", path),
            Err(e) => eprintln!("❌ Failed to move '{}': {}", path, e),
        }
    }
}
