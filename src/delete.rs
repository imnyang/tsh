use trash;

pub fn run_delete(paths: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    for path in paths {
        match trash::delete(path) {
            Ok(_) => println!("✅ {}", path),
            Err(e) => eprintln!("❌ Failed to delete {}: {}", path, e),
        }
    }
    Ok(())
}
