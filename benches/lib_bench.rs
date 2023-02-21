use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_library::Matrix;
use rand::{self, Rng};

const SIZE: usize = 5;
fn test_eq() -> bool {
	let array = [[rand::thread_rng().gen::<f32>(); SIZE]; SIZE];
	let second_array = [[rand::thread_rng().gen::<f32>(); SIZE]; SIZE];
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

fn matrix_math_bench(c: &mut Criterion) {
    c.bench_function("math", |b| b.iter(|| test_eq()));
}

criterion_group!(benches, matrix_math_bench);
criterion_main!(benches);
