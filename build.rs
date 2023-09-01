fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/main.rs", [&path]).build()?;
    b.flag_if_supported("-std=c++20")
        .file("src/cpp/input.cpp")
        .compile("autocxx-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cpp/input.h");
    println!("cargo:rerun-if-changed=src/cpp/input.cpp");
    Ok(())
}
