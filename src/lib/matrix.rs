use crate::Vector;
use std::{
    collections::{binary_heap::Iter, HashMap},
    default,
    fs::DirEntry,
    iter::{zip, Filter, Skip, TakeWhile},
    ops::{self, Add, AddAssign, Index, IndexMut, Neg, Rem, Sub},
    rc, vec,
};

const MATRIX_COUNT_ROWS: usize = 4;
const MATRIX_COUNT_COLS: usize = 4;

type ElementMatrix = f32;

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum Direction {
    #[default]
    Row,
    Col,
}

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct PosElem {
    pub row: usize,
    pub col: usize,
}
impl PosElem {
    pub fn new(row: usize, col: usize) -> Self {
        PosElem { row, col }
    }
    pub fn square(size: usize) -> Self {
        PosElem {
            row: size,
            col: size,
        }
    }
    pub fn Row(row: usize) -> Self {
        PosElem { row: row, col: 0 }
    }
    pub fn Col(col: usize) -> Self {
        PosElem { row: 0, col: col }
    }
    pub fn index2pos(&self, index: usize, direction: &Direction) -> PosElem {
        match direction {
            Direction::Row => (index / self.col, index % self.col),
            Direction::Col => (index % self.row, index / self.row),
        }
        .into()
    }
    pub fn pos2index(&self, pos: PosElem, direction: Direction) -> usize {
        match direction {
            Direction::Row => pos.row * self.col + pos.col,
            Direction::Col => pos.col * self.row + pos.row,
        }
    }
    pub fn iter(&self, direction: Direction) -> MatTraverse {
        MatTraverse::new(self.clone(), direction)
    }
}
impl From<(usize, usize)> for PosElem {
    fn from(pos: (usize, usize)) -> Self {
        PosElem::new(pos.0, pos.1)
    }
}
impl From<usize> for PosElem {
    fn from(size: usize) -> Self {
        PosElem::square(size)
    }
}
impl From<PosElem> for usize {
    fn from(pos: PosElem) -> Self {
        pos.row * pos.col
    }
}
impl Add for PosElem {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.row + rhs.row, self.col + rhs.col).into()
    }
}
impl AddAssign for PosElem {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}
impl AddAssign<usize> for PosElem {
    fn add_assign(&mut self, rhs: usize) {
        self.row += rhs;
        self.col += rhs;
    }
}
impl Add<usize> for PosElem {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        (self.row + rhs, self.col + rhs).into()
    }
}
impl Sub for PosElem {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.row - rhs.row, self.col - rhs.col).into()
    }
}
impl Sub<usize> for PosElem {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        (self.row - rhs, self.col - rhs).into()
    }
}

#[derive(PartialEq, Debug)]
pub struct MatTraverse {
    current_element: usize,
    count: PosElem,
    direction: Direction,
    end: usize,
}
impl Iterator for MatTraverse {
    type Item = PosElem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_element >= self.end {
            return None;
        }
        let pos = self.count.index2pos(self.current_element, &self.direction);
        self.current_element += 1;
        Some(pos)
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current_element = n;
        self.next()
    }
}

impl MatTraverse {
    fn new(count: PosElem, direction: Direction) -> Self {
        Self {
            current_element: 0,
            count,
            direction,
            end: count.into(),
        }
    }
    pub fn skip_pos(self, pos: PosElem) -> Skip<MatTraverse> {
        let index = self.count.pos2index(pos, self.direction);
        self.skip(index)
    }
}

#[derive(Debug)]
pub struct Matrix {
    content: [[ElementMatrix; MATRIX_COUNT_COLS]; MATRIX_COUNT_ROWS],
    pub count: PosElem,
    direction: Direction,
}
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.count.row != other.count.row || self.count.col != other.count.col {
            return false;
        }

        for pos in self.count.iter(Direction::Row) {
            if self[pos] != other[pos] {
                return false;
            }
        }
        return true;
    }
}
impl ops::Index<PosElem> for Matrix {
    type Output = ElementMatrix;
    fn index(&self, index: PosElem) -> &Self::Output {
        &self.content[index.row][index.col]
    }
}
impl ops::Index<(usize, usize)> for Matrix {
    type Output = ElementMatrix;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self[PosElem::from(index)]
    }
}
impl ops::Index<usize> for Matrix {
    type Output = ElementMatrix;
    fn index(&self, index: usize) -> &Self::Output {
        &self[PosElem::from(self.count.index2pos(index, &Direction::Row))]
    }
}
impl ops::IndexMut<PosElem> for Matrix {
    fn index_mut(&mut self, index: PosElem) -> &mut Self::Output {
        &mut self.content[index.row][index.col]
    }
}
impl ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let pos: PosElem = self.count.index2pos(index, &Direction::Row).into();
        &mut self[pos]
    }
}
impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        &mut self[PosElem::from(pos)]
    }
}
impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut result: Matrix = Matrix::new((self.count.row, rhs.count.col).into(), 0f32);

        for pos_l in self.count.iter(Direction::Row) {
            for pos_r in rhs
                .count
                .iter(Direction::Row)
                .take_while(|p| p.row == pos_l.col)
            {
                result[(pos_l.row, pos_r.col)] += self[pos_l] * rhs[pos_r];
            }
        }
        result
    }
}
impl ToString for Matrix {
    fn to_string(&self) -> String {
        let mut str = String::new();

        let mut last_row: usize = 0;
        for pos in self.count.iter(Direction::Row) {
            if pos.row != last_row {
                str.push('\n');
                last_row = pos.row;
            }
            str.push_str(&format!("{} ", self[pos]))
        }
        str
    }
}

impl Matrix {
    pub fn new(count: PosElem, value: f32) -> Self {
        Self {
            content: [[value; MATRIX_COUNT_COLS]; MATRIX_COUNT_ROWS],
            count,
            direction: Direction::Row,
        }
    }
    pub fn test_matrix(value: f32) -> Self {
        Self::new((3, 3).into(), value)
    }
    pub extern "C" fn is_square(&self) -> bool {
        self.count.col == self.count.row
    }
    pub fn minor(&self, exc_el: PosElem) -> Matrix {
        let mut matrix = Matrix::new(self.count - 1, 0.0);
        let mut new_pos: usize = 0;
        for pos in self.count.iter(Direction::Row) {
            if pos.row == exc_el.row || pos.col == exc_el.col {
                continue;
            }
            matrix[new_pos] = self[pos];
            new_pos += 1;
        }
        matrix
    }

    fn find_row_or_col_with_zero(&self) -> Option<Box<dyn Iterator<Item = PosElem>>> {
        const BASE_SIZE: usize = 5;
        let mut count_zero = (
            HashMap::<usize, usize>::with_capacity(BASE_SIZE),
            HashMap::<usize, usize>::with_capacity(BASE_SIZE),
        );
        for pos in self.count.iter(Direction::Row) {
            if self[pos] == 0.0 {
                *count_zero.0.entry(pos.row).or_insert(0) += 1;
                *count_zero.1.entry(pos.col).or_insert(0) += 1;
            }
        }
        if count_zero.0.is_empty() {
            return None;
        }
        let mut max_count: ((usize, usize), (usize, usize)) = ((0, 0), (0, 0));
        for (row, col) in zip(count_zero.0, count_zero.1) {
            if max_count.0 .1 < row.1 {
                max_count.0 = row;
            }
            if max_count.1 .1 < col.1 {
                max_count.1 = col;
            }
        }
        let res: Box<dyn Iterator<Item = PosElem>> = if max_count.0 .1 >= max_count.1 .1 {
            Box::new(
                self.count
                    .iter(Direction::Row)
                    .skip_pos((max_count.0 .0, 0).into())
                    .take_while(move |p| p.row == max_count.0 .0),
            )
        } else {
            Box::new(
                self.count
                    .iter(Direction::Col)
                    .skip_pos((max_count.0 .0, 0).into())
                    .take_while(move |p| p.col == max_count.1 .0),
            )
        };
        Some(res)
    }
    pub extern "C" fn determinant(&self) -> f32 {
        if self.is_square() == false {
            panic!()
        } else if self.count.row == 1 {
            return self[(0, 0)];
        }
        let target: Box<dyn Iterator<Item = PosElem>> = self.find_row_or_col_with_zero().unwrap_or(
            Box::new(self.count.iter(Direction::Row).take_while(|p| p.row == 0)),
        );
        let mut result = 0f32;
        for pos in target {
            let power = if (pos.row + pos.col) % 2 == 1 {
                -1f32
            } else {
                1f32
            };
            result += self.minor(pos).determinant() * power
        }
        return result;
    }
    pub fn elem_is_exist(&self, pos: PosElem) -> bool {
        pos.row < self.count.row || pos.col < self.count.col
    }
}
impl From<&[&[ElementMatrix]]> for Matrix {
    fn from(arr: &[&[ElementMatrix]]) -> Self {
        let count: PosElem = (arr.len(), arr[0].len()).into();
        let mut content = [[0f32; MATRIX_COUNT_COLS]; MATRIX_COUNT_ROWS];
        for pos in count.iter(Direction::Row) {
            content[pos.row][pos.col] = arr[pos.row][pos.col]
        }
        Matrix {
            content,
            count,
            direction: Direction::Row,
        }
    }
}
const SIZE_AUGMENTED_MATRIX: usize = 3;
pub struct AugmentedMatrix {
    matrices: [Matrix; 2],
    pub count: PosElem,
}
impl AugmentedMatrix {
    pub fn new(matrices: [Matrix; 2]) -> Self {
        let count = (
            matrices[0].count.row,
            matrices[0].count.col + matrices[1].count.col,
        )
            .into();
        Self { matrices, count }
    }
    pub fn SLAE(&mut self) {
        let mut start: PosElem = 0.into();

        while start.row < self.count.row && start.col < self.count.col {
            let mut start_value = self[start];

            print!("Resolution number - {}", start_value);
            if start_value == 0f32 {
                println!(" (pass)");
                start += 1;
                continue;
            }
            print!("\n{} row: ", start.row);
            self.count
                .iter(Direction::Row)
                .skip_pos(start)
                .take_while(|p| p.row == start.row)
                .for_each(|p| {
                    print!("{} -> ", self[p]);
                    self[p] /= start_value;
                    print!("{}; ", self[p]);
                });
            println!("\n{}", self.to_string());

            for pos_2 in self
                .count
                .iter(Direction::Row)
                .filter(|p| p.row != start.row && p.col >= start.col)
            {
                if start.col == pos_2.col {
                    start_value = self[pos_2]
                };
                let pos = (start.row, pos_2.col).into();
                println!(
                    "{:?} - {:?} * {} = {:?}",
                    self[pos_2],
                    self[pos],
                    start_value,
                    self[pos_2] - self[pos] * start_value
                );
                self[pos_2] -= self[pos] * start_value;
            }
            start += 1;
        }
    }
}
impl ToString for AugmentedMatrix {
    fn to_string(&self) -> String {
        let mut str = String::new();

        let mut last_row: usize = 0;
        for pos in self.count.iter(Direction::Row) {
            if pos.row != last_row {
                str.push('\n');
                last_row = pos.row;
            }
            str.push_str(&format!("{} ", self[pos]))
        }
        str
    }
}
impl IndexMut<PosElem> for AugmentedMatrix {
    fn index_mut(&mut self, mut pos: PosElem) -> &mut Self::Output {
        if pos.col >= self.matrices[0].count.col {
            pos.col %= self.matrices[0].count.col - 1;
            &mut self.matrices[1][pos]
        } else {
            &mut self.matrices[0][pos]
        }
    }
}
impl Index<PosElem> for AugmentedMatrix {
    type Output = ElementMatrix;
    fn index(&self, mut pos: PosElem) -> &Self::Output {
        if pos.col >= self.matrices[0].count.col {
            pos.col %= self.matrices[0].count.col - 1;
            &self.matrices[1][pos]
        } else {
            &self.matrices[0][pos]
        }
    }
}
#[test]
fn find_zero_test() {
    let mut matrix = Matrix::test_matrix(1f32);
    assert_eq!(matrix.find_row_or_col_with_zero().is_none(), true);
    matrix[(0, 0)] = 0f32;
    assert_eq!(matrix.find_row_or_col_with_zero().is_some(), true);
}
#[test]
fn mat_mul() {
    let lhs = Matrix::new(PosElem::new(3, 2), 1f32);
    let rhs = Matrix::new(PosElem::new(2, 3), 1f32);
    let result = &lhs * &rhs;
    assert_eq!(result.count, 3.into());
}
#[test]
fn mat_traverse() {
    let mut matrix = Matrix::test_matrix(1f32);
    for pos in matrix.count.iter(Direction::Row) {
        assert_eq!(matrix[pos], 1f32);
    }
    let mut count = 0;
    for pos in matrix.count.iter(Direction::Col).take_while(|p| p.col == 0) {
        count += 1;
    }
    assert_eq!(count, 3);
}
#[test]
fn test_determinant() {
    let mat = Matrix::test_matrix(1f32);
    assert_eq!(mat.determinant(), 0f32);
}
#[test]
fn test_augmented_matrix() {
    let mat = AugmentedMatrix::new([Matrix::test_matrix(1f32), Matrix::new((3, 1).into(), 1f32)]);
    let mut rows = 0;
    for pos in mat.count.iter(Direction::Row).take_while(|p| p.row == 0) {
        rows += 1;
    }
    assert_eq!(rows, 4);
}
