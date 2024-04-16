use alloc::vec::Vec;
use core::ops::{Add, Mul, Sub};

/// This is a basic building block for DSP math and operations
#[derive(Debug)]
pub struct Matrix<T>
{
    pub cpu_matrix: Vec<Vec<T>>
}

impl<T> Matrix<T>
    where T: Copy,
{
    /// This loads a 2d array made as a slice to a matrix
    pub fn from_slice(slice: &[&[T]]) -> Matrix<T>{
        let mut arr = Vec::new();

        for x in slice{
            arr.push(x.to_vec());
        }

        Matrix{
            cpu_matrix: arr,
        }
    }

    /// This loads a 2d array made as a vector to a matrix
    pub fn from_vec(vec: Vec<Vec<T>>) -> Matrix<T>{
        Matrix{
            cpu_matrix: vec,
        }
    }
}

impl<T> Mul for Matrix<T>
    where T: Copy + Mul<T, Output = T> + Add<T, Output = T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self {
        debug_assert_eq!(self.cpu_matrix[0].len(), rhs.cpu_matrix.len());

        let mut output = Vec::new();

        for this_row_index in 0..self.cpu_matrix.len(){

            let mut new_row = Vec::new();

            for other_col_index in 0 .. rhs.cpu_matrix[0].len(){
                for other_row_index in 0..rhs.cpu_matrix.len(){
                    if other_row_index == 0{
                        new_row.push(self.cpu_matrix[this_row_index][0] * rhs.cpu_matrix[0][other_col_index]);
                    }else {
                        new_row[other_col_index] = new_row[other_col_index] + (self.cpu_matrix[this_row_index][other_row_index] * rhs.cpu_matrix[other_row_index][other_col_index]);
                    }

                }

            }
            
            output.push(new_row)
        }

        Matrix{
            cpu_matrix: output
        }
    }
}

impl<T> Add for Matrix<T>
    where T: Copy + Add<T, Output = T>
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self {
        Matrix{
            cpu_matrix: self.cpu_matrix
                .iter()
                .enumerate()
                .map(
                    |(r_index,row)|
                        row.iter().enumerate().map(|(c_index, &col)| col + rhs.cpu_matrix[r_index][c_index])
                            .collect()
                )
                .collect(),
        }
    }
}

impl<T> Sub for Matrix<T>
    where T: Copy + Sub<T, Output = T>
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self {
        Matrix{
            cpu_matrix: self.cpu_matrix
                .iter()
                .enumerate()
                .map(
                    |(r_index,row)|
                        row.iter().enumerate().map(|(c_index, &col)| col - rhs.cpu_matrix[r_index][c_index])
                            .collect()
                )
                .collect(),
        }
    }
}

impl<T> PartialEq for Matrix<T>
    where T: Copy + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        for (index, x) in self.cpu_matrix.iter().enumerate(){
            if *x != other.cpu_matrix[index]{
                return false
            }
        }
        
        true
    }
}

impl<T> Clone for Matrix<T>
    where T: Copy + PartialEq
{
    fn clone(&self) -> Self {
        Matrix{
            cpu_matrix: self.cpu_matrix.clone()
        }
    }
}
