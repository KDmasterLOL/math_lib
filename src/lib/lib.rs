pub mod tokeniser;
pub mod math;
pub mod matrix;
pub use matrix::Matrix;
#[macro_use]
extern crate lazy_static;

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello")
}
#[test]
pub fn minor_test() {
    let matrix = Matrix::new((3, 3).into(), 1.0);

    assert_eq!(matrix.minor((0, 0).into()), Matrix::new((2, 2).into(), 1.0));
}
