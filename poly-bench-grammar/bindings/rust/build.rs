//! Build script for tree-sitter-polybench
//!
//! This compiles the Tree-sitter parser and custom scanner.
//!
//! Note: The parser.c file must be generated first by running:
//!   cd poly-bench-grammar && npm install && npm run generate

fn main() {
    let src_dir = std::path::Path::new("src");

    let parser_path = src_dir.join("parser.c");
    if !parser_path.exists() {
        eprintln!("=======================================================");
        eprintln!("ERROR: src/parser.c not found!");
        eprintln!("");
        eprintln!("The Tree-sitter grammar needs to be generated first.");
        eprintln!("Run the following commands:");
        eprintln!("");
        eprintln!("  cd poly-bench-grammar");
        eprintln!("  npm install");
        eprintln!("  npm run generate");
        eprintln!("");
        eprintln!("Or use: make grammar");
        eprintln!("=======================================================");
        std::process::exit(1);
    }

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);

    // Enable UTF-8 source encoding for MSVC
    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    // Compile the generated parser
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    // Compile the custom scanner
    let scanner_path = src_dir.join("scanner.c");
    if scanner_path.exists() {
        c_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    }

    c_config.compile("tree-sitter-polybench");
}
