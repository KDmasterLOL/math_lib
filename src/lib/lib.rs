pub mod equation;
pub mod math;
pub mod vector;
pub mod matrix;
pub mod AugmentedMatrix;
pub use matrix::Matrix;
pub use vector::Vector;

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello")
}
#[test]
pub fn minor_test() {
    let matrix = Matrix::new((3, 3).into(), 1.0);

    assert_eq!(matrix.minor((0, 0).into()), Matrix::new((2, 2).into(), 1.0));
}
