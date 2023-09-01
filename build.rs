use std::env;

fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    env::set_var("CROSS_COMPILE ", "1");

    let mut b = autocxx_build::Builder::new("src/main.rs", [&path])
        .extra_clang_args(&["-std=c++20", "-stdlib=libc++", "--x86_64-pc-windows-gnu"])
        .build()?;

    b.cpp(true)
        .std("c++20")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-stdlib=libc++")
        .flag_if_supported("--target=x86_64-pc-windows-gnu")
        .target("x86_64-pc-windows-gnu")
        .file("src/cpp/input.cpp")
        .compile("autocxx-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cpp/input.h");
    println!("cargo:rerun-if-changed=src/cpp/input.cc");
    Ok(())
}
