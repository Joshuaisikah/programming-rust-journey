// Ch12 — Operator Overloading | Ch11 — Traits & Generics
//
// CONCEPTS:
//   Add trait          — implement + between two Matrix values
//   Mul trait          — implement * for matrix multiplication
//   Index trait        — implement matrix[row][col] syntax
//   Display trait      — pretty-print the matrix
//   PartialEq          — element-wise equality comparison
//
// NOTE ON INDEX:
//   matrix[i] returns &[f64] (a row slice).
//   matrix[i][j] then indexes into that slice — standard Rust pattern.

use std::fmt;
use std::ops::{Add, Index, Mul};

// ── Matrix ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    data: Vec<f64>, // row-major: data[r * cols + c]
}

impl Matrix {
    /// Create a `rows × cols` zero matrix.
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Create a matrix from a flat row-major Vec.
    /// Panics if data.len() != rows * cols.
    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
      assert_eq!(data.len(), rows * cols);
        Matrix {
            rows,
            cols,
            data
        }
    }

    /// Return the element at (row, col).
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    /// Set the element at (row, col).
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.data[row * self.cols + col] = val
    }

    /// Return the transpose: a `cols × rows` matrix.
    pub fn transpose(&self) -> Self {
        let mut transposed = Matrix::new(self.cols,self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                transposed.set(c, r, self.get(r, c));
            }
        }
        transposed
    }

    /// Return the identity matrix of size `n × n`.
    pub fn identity(n: usize) -> Self {
        let mut id = Matrix::new(n, n);
        for i in 0..n {
            id.set(i, i, 1.0);
        }
        id

    }
}

// ── Add ───────────────────────────────────────────────────────

impl Add for Matrix {
    type Output = Matrix;
    /// Element-wise addition. Panics if dimensions don't match.
    fn add(self, rhs: Matrix) -> Matrix {
       assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.rows, rhs.rows);
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let sum = self.get(i, j) + rhs.get(i, j);
                result.set(i, j, sum);
            }
        }
        result
    }
}

// ── Mul (matrix multiplication) ───────────────────────────────

impl Mul for Matrix {
    type Output = Matrix;
    /// Standard matrix multiplication (rows × cols) · (cols × k) → (rows × k).
    fn mul(self, rhs: Matrix) -> Matrix {
        // Check that inner dimensions match
        assert_eq!(self.cols, rhs.rows, "Left cols must equal right rows");

        // Result has dimensions: self.rows × rhs.cols
        let mut result = Matrix::new(self.rows, rhs.cols);

        // Triple nested loop: for each result position (i,j), sum over k
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {  // or rhs.rows, since they're equal
                    sum += self.get(i, k) * rhs.get(k, j);
                }
                result.set(i, j, sum);
            }
        }

        result
    }
}

// ── Index ─────────────────────────────────────────────────────

impl Index<usize> for Matrix {
    type Output = [f64];
    /// Return the r-th row as a slice. Enables `matrix[r][c]` syntax.
    fn index(&self, row: usize) -> &[f64] {
        &self.data[row * self.cols .. (row + 1) * self.cols]
    }
}

// ── Display ───────────────────────────────────────────────────

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                write!(f, "{} ", self.get(r, c))?;  // Write value + space
            }
            writeln!(f)?;  // Newline after each row
        }
        Ok(())
    }
}

// ── Demo ──────────────────────────────────────────────────────

pub fn demo() {
    println!("=== Matrix demo ===");
    let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    println!("A:\n{}", a);
    println!("B:\n{}", b);
    println!("A + B:\n{}", a.clone() + b.clone());
    println!("A * B:\n{}", a.clone() * b.clone());
    println!("A transposed:\n{}", a.transpose());
    println!("3×3 identity:\n{}", Matrix::identity(3));
    println!("A[0][1] = {}", a[0][1]);
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_all_zeros() {
        let m = Matrix::new(2, 3);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 3);
        for r in 0..2 {
            for c in 0..3 {
                assert_eq!(m.get(r, c), 0.0);
            }
        }
    }

    #[test]
    fn test_from_vec_and_get() {
        let m = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 1), 2.0);
        assert_eq!(m.get(1, 0), 3.0);
        assert_eq!(m.get(1, 1), 4.0);
    }

    #[test]
    fn test_set_updates_element() {
        let mut m = Matrix::new(2, 2);
        m.set(1, 1, 9.0);
        assert_eq!(m.get(1, 1), 9.0);
        assert_eq!(m.get(0, 0), 0.0); // others unchanged
    }

    #[test]
    fn test_transpose_swaps_dims() {
        let m = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let t = m.transpose();
        assert_eq!(t.rows, 3);
        assert_eq!(t.cols, 2);
        assert_eq!(t.get(0, 0), 1.0);
        assert_eq!(t.get(1, 0), 2.0);
        assert_eq!(t.get(2, 0), 3.0);
    }

    #[test]
    fn test_identity_diagonal_is_one() {
        let id = Matrix::identity(3);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_eq!(id.get(i, j), expected);
            }
        }
    }

    #[test]
    fn test_add_element_wise() {
        let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let c = a + b;
        assert_eq!(c.get(0, 0), 6.0);
        assert_eq!(c.get(1, 1), 12.0);
    }

    #[test]
    fn test_multiply_2x2() {
        // [[1,2],[3,4]] × [[5,6],[7,8]] = [[19,22],[43,50]]
        let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let c = a * b;
        assert_eq!(c.get(0, 0), 19.0);
        assert_eq!(c.get(0, 1), 22.0);
        assert_eq!(c.get(1, 0), 43.0);
        assert_eq!(c.get(1, 1), 50.0);
    }

    #[test]
    fn test_index_row_then_col() {
        let m = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[1][0], 3.0);
    }
}
