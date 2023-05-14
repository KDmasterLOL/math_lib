use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_library::{Matrix};
use rand::{self, Rng};
macro_rules! slice_array {
    ($($elem:expr),* $(,)?) => {
        [$($elem.as_slice()),*].as_slice()
     }
}
const SIZE: usize = 5;
fn test_eq() -> bool {
    let array = [[rand::thread_rng().gen::<f64>(); SIZE]; SIZE];
    let second_array = [[rand::thread_rng().gen::<f64>(); SIZE]; SIZE];
    // let mut eq = true;
    // for r in 0..5 {
    //     for c in 0..5 {
    //         if array[r][c] != second_array[r][c] {
    //             eq = false;
    //         }
    //     }
    // }
    // eq
    array == second_array
}
fn test_speed_basis() {
    let mut mat_1 = Matrix::from(slice_array!(
        [1f64, 3f64, 4f64],
        [1f64, 2f64, 4f64],
        [1f64, 9f64, 10f64]
    ));
    let mut mat_2 = Matrix::from(slice_array!([1f64], [31f64], [42f64]));
}

fn matrix_math_bench(c: &mut Criterion) {
    c.bench_function("math", |b| b.iter(|| test_speed_basis()));
}

criterion_group!(benches, matrix_math_bench);
criterion_main!(benches);
