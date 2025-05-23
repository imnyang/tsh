# 🗑️ tsh - Move files to Trash Bin

`tsh` is a simple command-line tool written in Rust that moves files and folders to the **Trash Bin**, instead of permanently deleting them.

## 🔧 Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```sh
cargo install tsh
```

If installing from a local directory:

```sh
git clone https://github.com/imnyang/tsh.git
cd tsh
cargo install --path .
```

## 🚀 Usage
```sh
tsh <file_or_folder> [more_files_or_folders...]
```

```sh
tsh file.txt
tsh folder/ notes.md image.png
```

## 📦 Features
- ✅ Safe delete: files go to Trash, not permanently removed.

- 🔧 Built on the reliable [trash](https://crates.io/crates/trash) crate.

