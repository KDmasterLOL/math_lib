use std::{
    collections::HashMap,
    fmt::Display,
    iter::{zip, Skip},
    ops::{Add, AddAssign, Index, IndexMut, Mul, Sub},
};

pub const MATRIX_COUNT_ROWS: usize = 8;
pub const MATRIX_COUNT_COLS: usize = 8;

type ElementMatrix = f64;

#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum Direction {
    #[default]
    Row,
    Col,
}
#[repr(C)]
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct PosElem {
    pub row: usize,
    pub col: usize,
}
impl PosElem {
    pub fn zero() -> Self {
        Self { row: 0, col: 0 }
    }
    pub fn new(row: usize, col: usize) -> Self {
        PosElem { row, col }
    }
    pub fn square(size: usize) -> Self {
        PosElem {
            row: size,
            col: size,
        }
    }
    pub fn row(row: usize) -> Self {
        PosElem { row: row, col: 0 }
    }
    pub fn col(col: usize) -> Self {
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
    pub fn iter_skip(&self, direction: Direction, pos: PosElem) -> Skip<MatTraverse> {
        MatTraverse::new(self.clone(), direction).skip(self.pos2index(pos, direction))
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
impl AddAssign<(isize, isize)> for PosElem {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        self.row = self
            .row
            .checked_add_signed(rhs.0)
            .expect("Overflow addition of PosElem and isize in  AddAssign");
        self.col = self
            .col
            .checked_add_signed(rhs.1)
            .expect("Overflow addition of PosElem and isize in  AddAssign");
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
impl Display for PosElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row: {}, col: {})", self.row, self.col)
    }
}
#[repr(C)]
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
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    content: [[ElementMatrix; MATRIX_COUNT_COLS]; MATRIX_COUNT_ROWS],
    pub count: PosElem,
    direction: Direction,
}
impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut last_row: usize = 0;
        str.push_str("Matrix:\n");
        for pos in self.count.iter(Direction::Row) {
            if pos.row != last_row {
                str.push('\n');
                last_row = pos.row;
            }
            str.push_str(&format!("{:.3}\t", self[pos]))
        }
        write!(f, "{}", str)
    }
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
impl Index<PosElem> for Matrix {
    type Output = ElementMatrix;
    fn index(&self, index: PosElem) -> &Self::Output {
        &self.content[index.row][index.col]
    }
}
impl Index<usize> for Matrix {
    type Output = ElementMatrix;
    fn index(&self, index: usize) -> &Self::Output {
        &self[PosElem::from(self.count.index2pos(index, &Direction::Row))]
    }
}
impl IndexMut<PosElem> for Matrix {
    fn index_mut(&mut self, index: PosElem) -> &mut Self::Output {
        &mut self.content[index.row][index.col]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let pos: PosElem = self.count.index2pos(index, &Direction::Row).into();
        &mut self[pos]
    }
}
impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut result: Matrix = Matrix::new(
            (self.count.row, rhs.count.col).into(),
            ElementMatrix::default(),
        );

        for pos_l in self.count.iter(Direction::Row) {
            for pos_r in rhs
                .count
                .iter(Direction::Row)
                .take_while(|p| p.row == pos_l.col)
            {
                result[PosElem::new(pos_l.row, pos_r.col)] += self[pos_l] * rhs[pos_r];
            }
        }
        result
    }
}

impl Matrix {
    #[export_name = "init"]
    pub extern "C" fn new(count: PosElem, value: ElementMatrix) -> Self {
        Self {
            content: [[value; MATRIX_COUNT_COLS]; MATRIX_COUNT_ROWS],
            count,
            direction: Direction::Row,
        }
    }
    pub fn augmented_matrix(main_matrix: &[&[ElementMatrix]], constant: &[ElementMatrix]) -> Self {
        let count = PosElem::new(main_matrix.len(), main_matrix[0].len() + 1);
        if constant.len() != count.row {
            panic!("AAA AUG");
        }

        let mut augmented_matrix = Self::new(count, ElementMatrix::default());
        for pos in augmented_matrix.count.iter(Direction::Row) {
            augmented_matrix[pos] = if pos.col + 1 == augmented_matrix.count.col {
                constant[pos.row]
            } else {
                main_matrix[pos.row][pos.col]
            };
        }
        augmented_matrix
    }

    pub fn new_simplex_table(
        criterian_function: &[ElementMatrix],
        constraints: &[(char, ElementMatrix)],
        coefficient_table: &[&[ElementMatrix]],
    ) -> Self {
        let count = PosElem::new(constraints.len() + 1, criterian_function.len() + 1);
        let mut simplex_table = Self::new(count, ElementMatrix::default());
        for pos in count.iter(Direction::Row) {
            simplex_table[pos] = if pos == PosElem::zero() {
                ElementMatrix::default()
            } else if pos.row == 0 {
                criterian_function[pos.col - 1]
            } else if pos.col == 0 {
                if constraints[pos.row - 1].0 != '=' {
                    simplex_table.offset_size((0, 1).into());
                    let target_pos = PosElem::new(pos.row, simplex_table.count.col - 1);
                    simplex_table[target_pos] = if constraints[pos.row - 1].0 == '≥' {
                        -1f64
                    } else {
                        1f64
                    }
                }
                constraints[pos.row - 1].1
            } else {
                coefficient_table[pos.row - 1][pos.col - 1]
            }
        }
        simplex_table
    }
    pub fn new_simplex_table_with_artificial_cf(
        criterian_function: &[ElementMatrix],
        artificial_cf: &[ElementMatrix],
        constraints: &[ElementMatrix],
        coefficient_table: &[&[ElementMatrix]],
    ) -> Self {
        let count = PosElem::new(criterian_function.len() + 2, constraints.len() + 1);
        let mut simplex_table = Self::new(count, ElementMatrix::default());
        for pos in count.iter(Direction::Row) {
            simplex_table[pos] = if pos.row == 0 {
                criterian_function[pos.col]
            } else if pos.row == 1 {
                artificial_cf[pos.col]
            } else if pos.col == 0 {
                constraints[pos.row - 1]
            } else {
                coefficient_table[pos.row - 1][pos.col - 1]
            }
        }
        simplex_table
    }
    #[no_mangle]
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
    pub fn offset_size(&mut self, offset: PosElem) {
        self.count += offset;
    }
    pub fn resize(&mut self, new_count: PosElem) {
        self.count = new_count;
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
                    .iter_skip(Direction::Row, (max_count.0 .0, 0).into())
                    .take_while(move |p| p.row == max_count.0 .0),
            )
        } else {
            Box::new(
                self.count
                    .iter_skip(Direction::Col, (max_count.0 .0, 0).into())
                    .take_while(move |p| p.col == max_count.1 .0),
            )
        };
        Some(res)
    }
    #[no_mangle]
    pub extern "C" fn determinant(&self) -> ElementMatrix {
        if self.is_square() == false {
            panic!()
        } else if self.count.row == 1 {
            return self[PosElem::zero()];
        }
        let target: Box<dyn Iterator<Item = PosElem>> = self.find_row_or_col_with_zero().unwrap_or(
            Box::new(self.count.iter(Direction::Row).take_while(|p| p.row == 0)),
        );
        let mut result = ElementMatrix::default();
        for pos in target {
            let power = if (pos.row + pos.col) % 2 == 1 {
                -1f64
            } else {
                1f64
            };
            result += self.minor(pos).determinant() * power
        }
        return result;
    }
    pub fn gauss(&mut self) {
        let mut basises = Vec::<PosElem>::new();
        let count_main_matrix = PosElem::new(self.count.row, self.count.col - 1);
        for pos in count_main_matrix.iter(Direction::Col) {
            if self[pos] == ElementMatrix::default()
                || basises.iter().any(|p| p.row == pos.row || p.col == pos.col)
            {
                continue;
            }
            self.to_basis(pos);
            basises.push(pos);
        }
    }

    pub fn to_basis(&mut self, pos_resolution_el: PosElem) {
        let value = self[pos_resolution_el];
        self.count
            .iter_skip(Direction::Row, PosElem::new(pos_resolution_el.row, 0))
            .take_while(|pos| pos.row == pos_resolution_el.row)
            .for_each(|pos| self[pos] /= value);
        let mut resolution_col_el = ElementMatrix::default();
        let mut last_row = usize::MAX;
        for pos in self
            .count
            .iter(Direction::Row)
            .filter(|p| p.row != pos_resolution_el.row)
        {
            if last_row != pos.row {
                resolution_col_el = self[PosElem::new(pos.row, pos_resolution_el.col)];
                last_row = pos.row;
            };
            let resolution_row_el = self[PosElem::new(pos_resolution_el.row, pos.col)];
            self[pos] -= resolution_col_el * resolution_row_el;
        }
    }
    pub fn find_basic_feasible_solution() -> Matrix {
        todo!()
    }
    pub fn simplex(&mut self, kriterian_row: usize) {
        let mut is_optimum = false;
        loop {
            println!("{}", self);
            let mut pos_resolution = PosElem::new(kriterian_row, 0);
            is_optimum = true;
            let mut max_value = ElementMatrix::default();
            for pos_kriterian_row in self
                .count
                .iter_skip(Direction::Row, PosElem::new(kriterian_row, 1))
                .take_while(|pos| pos.row == kriterian_row)
            {
                let kriterian_value = self[pos_kriterian_row];
                if kriterian_value.abs() > max_value
                    && kriterian_value.abs() > ElementMatrix::EPSILON
                    && self[pos_kriterian_row].signum() == -1.0
                {
                    pos_resolution.col = pos_kriterian_row.col;
                    max_value = self[pos_kriterian_row].abs();
                    is_optimum = false;
                }
            }
            if is_optimum == true {
                break;
            }
            for pos_resolution_col in self
                .count
                .iter_skip(Direction::Col, pos_resolution)
                .take_while(move |pos| pos.col == pos_resolution.clone().col)
            {
                if self[pos_resolution_col] <= ElementMatrix::default() {
                    continue;
                }
                let theta =
                    self[PosElem::new(pos_resolution_col.row, 0)] / self[pos_resolution_col];
                let theta_min = self[PosElem::new(pos_resolution.row, 0)] / self[pos_resolution];
                if self[pos_resolution] <= ElementMatrix::default() || theta < theta_min {
                    pos_resolution = pos_resolution_col;
                }
            }
            self.to_basis(pos_resolution);
        }
    }
    pub fn simplex_an(&mut self) {
        let mut is_optimum = false;
        loop {
            let mut pos_resolution = PosElem::zero();
            is_optimum = true;
            for kriterian_pos in self
                .count
                .iter(Direction::Row)
                .take_while(|pos| pos.row == 0)
            {
                if self[kriterian_pos] < ElementMatrix::default() {
                    pos_resolution.col = kriterian_pos.col;
                    is_optimum = false;
                    break;
                }
            }
            if is_optimum == true {
                break;
            }
            for pos_resolution_col in self
                .count
                .iter_skip(Direction::Col, pos_resolution)
                .take_while(move |pos| pos.col == pos_resolution.clone().col)
            {
                if self[pos_resolution_col] <= ElementMatrix::default() {
                    continue;
                }
                let theta = self[PosElem::new(pos_resolution_col.row, self.count.col - 1)]
                    / self[pos_resolution_col];
                let theta_min = self[PosElem::new(pos_resolution.row, self.count.col - 1)]
                    / self[pos_resolution];
                if self[pos_resolution] <= ElementMatrix::default() || theta < theta_min {
                    pos_resolution = pos_resolution_col;
                }
            }
            self.to_basis(pos_resolution);
        }
    }
}
impl From<&[&[ElementMatrix]]> for Matrix {
    fn from(arr: &[&[ElementMatrix]]) -> Self {
        let count: PosElem = (arr.len(), arr[0].len()).into();
        let mut content = [[ElementMatrix::default(); MATRIX_COUNT_COLS]; MATRIX_COUNT_ROWS];
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

// pub fn slae(&mut self) {
//     let mut last_pos = Vec::<PosElem>::with_capacity(self.count.row);
//     while last_pos.len() < self.count.row {
//         let mut start_value = ElementMatrix::default();
//         for pos in self.count.iter(Direction::Col).filter(|p| {
//             last_pos
//                 .iter()
//                 .any(|l_p| l_p.row == p.row || l_p.col == p.col)
//                 == false
//         }) {
//             if self[pos] != ElementMatrix::default() {
//                 last_pos.push(pos);
//                 start_value = self[pos];
//                 print!("Resolution number - {} at {}", start_value, pos);
//                 break;
//             }
//         }
//         if start_value == ElementMatrix::default() {
//             return;
//         }
//         let start: PosElem = last_pos.last().unwrap().clone();
//         print!("\nDiv {} row by {}: ", start.row, start_value);
//         self.count
//             .iter_skip(Direction::Row, start)
//             .take_while(|p| p.row == start.row)
//             .for_each(|p| {
//                 print!("{} -> ", self[p]);
//                 self[p] /= start_value;
//                 print!("{}; ", self[p]);
//             });
//         for pos_2 in self
//             .count
//             .iter(Direction::Row)
//             .filter(|p| p.row != start.row && p.col >= start.col)
//         {
//             if start.col == pos_2.col {
//                 println!("procces {} row", pos_2.row);
//                 start_value = self[pos_2]
//             };
//             let pos = (start.row, pos_2.col).into();
//             println!(
//                 "{:?} - {:?} * {} = {:?}",
//                 self[pos_2],
//                 self[pos],
//                 start_value,
//                 self[pos_2] - self[pos] * start_value
//             );
//             self[pos_2] -= self[pos] * start_value;
//         }
//         println!("\n{}", self);
//     }
// }
#[test]
fn find_zero_test() {
    let mut matrix = Matrix::new(3.into(), 1f64);
    assert_eq!(matrix.find_row_or_col_with_zero().is_none(), true);
    matrix[PosElem::zero()] = ElementMatrix::default();
    assert_eq!(matrix.find_row_or_col_with_zero().is_some(), true);
}
#[test]
fn mat_mul() {
    let lhs = Matrix::new(PosElem::new(3, 2), 1f64);
    let rhs = Matrix::new(PosElem::new(2, 3), 1f64);
    let result = &lhs * &rhs;
    assert_eq!(result.count, 3.into());
}
#[test]
fn mat_traverse() {
    let mut matrix = Matrix::new(3.into(), 1f64);
    for pos in matrix.count.iter(Direction::Row) {
        assert_eq!(matrix[pos], 1f64);
    }
    let mut count = 0;
    for pos in matrix.count.iter(Direction::Col).take_while(|p| p.col == 0) {
        count += 1;
    }
    assert_eq!(count, 3);
}
#[test]
fn test_determinant() {
    let mat = Matrix::new(3.into(), 1f64);
    assert_eq!(mat.determinant(), ElementMatrix::default());
}
