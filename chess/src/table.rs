use std::{
    fmt::Display,
    slice::{Iter, IterMut},
};

use arrayvec::ArrayVec;

/// A generic 2D table for general use in chess engines.
#[derive(Debug, Clone)]
pub struct Table<T, const CAP: usize> {
    data: ArrayVec<T, CAP>,
    width: usize,
    height: usize,
}

impl<T, const CAP: usize> Table<T, CAP> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: ArrayVec::<T, CAP>::new(),
            width,
            height,
        }
    }

    pub fn insert(&mut self, value: T, row: usize, col: usize) {
        self.data.insert(self.index(row, col), value);
    }

    pub fn at(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(self.index(row, col))
    }

    pub fn row(&self, row: usize) -> &[T] {
        let start_idx = self.index(row, 0);
        let end_idx = self.index(row, self.cols() - 1);
        // note, this is an inclusive range
        self.data.get(start_idx..end_idx).expect("Invalid range")
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn rows(&self) -> usize {
        self.height
    }

    pub fn cols(&self) -> usize {
        self.width
    }

    pub fn fill<ValueProducer: Fn(usize, usize) -> T>(&mut self, value_producer: ValueProducer) {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                self.insert(value_producer(row, col), row, col);
            }
        }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl<T, const CAP: usize> Default for Table<T, CAP> {
    fn default() -> Self {
        Self::new(64, 64)
    }
}

impl<T: Display, const CAP: usize> Display for Table<T, CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();

        for row in 0..self.rows() {
            let row_data = self.row(row);
            for (col, item) in row_data.iter().enumerate() {
                output.push_str(format!("{}", item).as_str());
                if col < self.cols() {
                    output.push(',');
                    output.push(' ');
                }
            }

            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use crate::table::Table;

    #[test]
    fn initialize_and_fill() {
        const SIZE: usize = 8;
        let iota = |row: usize, col: usize| -> usize { row * SIZE + col };

        let mut table = Table::<usize, 64>::new(8, 8);
        table.fill(iota);

        for row in 0..table.rows() {
            for col in 0..table.cols() {
                let val = table.at(row, col);
                assert!(val.is_some());
                assert_eq!(*val.unwrap(), iota(row, col));
            }
        }
    }
}
