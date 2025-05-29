#[cfg(target_os = "macos")]
fn main() {
    eprintln!("❌ macOS는 현재 지원되지 않습니다.");
}

#[cfg(not(target_os = "macos"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage:\n  tsh <file_or_folder>...     # 삭제\n  tsh --restore | -u          # 복구 UI");
        std::process::exit(1);
    }

    /*match args[0].as_str() {
        "--restore" | "-u" => restore::run_restore_ui()?,
        _ => delete::run_delete(&args)?,
    }*/

    Ok(())
}

mod delete;
//mod restore;