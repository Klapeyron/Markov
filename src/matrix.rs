use std::clone::Clone;
use std::fmt;

#[derive(PartialEq, Clone)]
pub struct Matrix<T> {
    x: usize,
    y: usize,
    data: Vec<Vec<T>>
}

impl<T> Matrix<T> {
    pub fn new(default_value: T, xsize: usize, ysize: usize) -> Matrix<T>
        where T: Clone {
        Matrix {
            data: vec![vec![default_value; xsize]; ysize],
            x: xsize,
            y: ysize
        }
    }

    fn is_in_range(self: &Matrix<T>, x: usize, y: usize) -> bool {
        return x < self.x && y < self.y;
    }

    pub fn set_state(self: &mut Matrix<T>, new_state: T, x: usize, y: usize) -> bool {
        if !self.is_in_range(x, y) {
            return false;
        }

        self.data[y][x] = new_state;
        return true;
    }

    pub fn read_state(self: &Matrix<T>, x: usize, y: usize) -> Option<&T> {
        if self.is_in_range(x, y) {
            return Some(&self.data[y][x]);
        }
        else {
            return None;
        }
    }

    pub fn matrix(self: &Matrix<T>) -> &Vec<Vec<T>> {
        &self.data
    }
}

impl<T> fmt::Debug for Matrix<T>
    where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");
        for row in self.data.iter() {
            write!(f, "\n");
            for elem in row.iter() {
                write!(f, "[{:?}], ", elem);
            }
        }
        write!(f, "]")
    }
}

#[test]
fn correct_size() {
    let world: Matrix<u64> = Matrix::new(666, 4, 3);

    assert_eq!(4, world.x);
    assert_eq!(3, world.y);
}

#[test]
fn return_none_if_accessed_out_of_range() {
    let world: Matrix<u64> = Matrix::new(666, 4, 3);

    assert_eq!(None, world.read_state(5,3));
    assert_eq!(None, world.read_state(4,4));
    assert_eq!(None, world.read_state(4,4));
}

#[test]
fn return_false_if_inserted_out_of_range() {
    let mut world: Matrix<u64> = Matrix::new(666, 4, 3);

    assert_eq!(false, world.set_state(777, 5, 3));
    assert_eq!(false, world.set_state(777, 4, 4));
}

#[test]
fn all_fields_are_initialized_corectly() {
    let world: Matrix<u64> = Matrix::new(666, 4, 3);
    let mut number_of_calls = 0;

    for row in world.matrix().iter() {
        for elem in row.iter() {
            number_of_calls += 1;
            assert_eq!(elem, &666);
        }
    }

    assert_eq!(4*3, number_of_calls);
}
