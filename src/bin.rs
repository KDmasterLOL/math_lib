use std::iter::zip;

use math_library::{
    self,
    matrix::{AugmentedMatrix, Direction, PosElem},
    Matrix,
};
macro_rules! matrix {
    ($row:block,$($other:tt)*) => { 
        $row.as_slice(),
        matrix!($($other)*)
      };
}

fn main() {
    let mat: Matrix = Matrix::from(
        [
            [1f32, 0f32, 1f32].as_slice(),
            [2f32, 4f32, 5f32].as_slice(),
            [2f32, 1f32, 5f32].as_slice(),
        ]
        .as_slice(),
    );
    // let mat: Matrix = Matrix::from(
    //     [
    //         matrix!([1f32, 0f32, 1f32],
    //         [2f32, 4f32, 5f32],
    //         [2f32, 1f32, 5f32]),
    //     ]
    //     .as_slice(),
    // );

    // println!("{:?}", );
    // let mut aug_mat = AugmentedMatrix::new([
    //     Matrix::new(2.into(), 5f32),
    //     Matrix::new((2, 1).into(), 3f32),
    // ]);
    // // .skip_pos(PosElem::Row(1)).take_while(|p|p.row==1)
    // println!("{}", aug_mat.to_string());
    // aug_mat.SLAE();
    // println!("{}", aug_mat.to_string());
}
fn lib_test() {
    let mut equation = math_library::equation::Equation::new(String::from("23+24+112"));
    equation.parse();
    println!("{:?}", equation.tokens)
}
