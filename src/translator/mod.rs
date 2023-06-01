use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use walkdir::{DirEntry, WalkDir};

use crate::{generate_rust, parser::parse_tree};

fn is_java(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".java"))
        .unwrap_or(false)
}
pub fn translate_java(src: &Path) -> anyhow::Result<()> {
    println!("source: {}", src.display());
    let walker = WalkDir::new(src).into_iter();
    // for entry in walker.filter_entry(is_java) {
    for entry in walker {
        let entry = entry?;
        println!("translating {}", entry.path().display());
        if entry.file_type().is_file() && entry.file_name().to_string_lossy().ends_with(".java") {
            translate_java_to_rust(entry.path())?;
        }
    }
    Ok(())
}
pub fn translate_java_to_rust(src: &Path) -> anyhow::Result<()> {
    println!("java file: {}", src.display());
    let mut java_file = fs::File::open(src)?;
    let mut java_src = String::new();
    java_file.read_to_string(&mut java_src)?;

    let source_tree = parse_tree(&java_src)?;

    let rust_path = src.with_extension("rs");
    let rust_file = fs::File::create(rust_path)?;
    let mut rust_writer = io::BufWriter::new(rust_file);

    generate_rust(&source_tree, &mut rust_writer)?;
    Ok(())
}
