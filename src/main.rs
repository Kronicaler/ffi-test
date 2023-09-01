use autocxx::prelude::*;

include_cpp! {
    #include "cpp/input.h"
    safety!(unsafe_ffi)
    generate!("DoMath")
    generate!("Goat")
    generate!("jurassic")
    generate!("print")
}

fn main() {
    println!("Rust math should say 12={}", 4 * 3);
    println!("C++ math called from Rust should say 12={}", ffi::DoMath(4));
    let mut goat = ffi::Goat::new().within_box();

    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();

    let describe = format!("This C++ goat has {} horns in Rust.", goat.get_horns());
    assert_eq!(
        goat.describe().as_ref().unwrap().to_string_lossy(),
        describe
    );
    println!("{}", describe);
    
    ffi::print();
    ffi::jurassic();
}

#[autocxx::extern_rust::extern_rust_function]
pub fn go_extinct() {
    println!("Rust Boom called from C++")
}
