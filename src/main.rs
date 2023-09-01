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
    println!("Hello, world! - C++ math should say 12={}", ffi::DoMath(4));
    let mut goat = ffi::Goat::new().within_box();

    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();
    goat.as_mut().add_a_horn();

    let describe = format!("This goat has {} horns.", goat.get_horns());
    assert_eq!(
        goat.describe().as_ref().unwrap().to_string_lossy(),
        describe
    );
    println!("{}", describe);

    ffi::jurassic();
    ffi::print();
}

#[autocxx::extern_rust::extern_rust_function]
pub fn go_extinct() {
    println!("Boom")
}
