use algebra_core::{Field, Zero, FftField};
use ff_fft::{DensePolynomial as Polynomial, 
            GeneralEvaluationDomain, 
            domain::{EvaluationDomain},
            evaluations::Evaluations,
            };
use core::marker::PhantomData;
use std::convert::TryFrom;

// Matrix encodings:
#[derive(Debug, Clone)]
pub struct Matrix<F> {
    pub rows: Vec::<Vec::<F>>,
}

#[derive(Debug)]
pub struct MatrixSparseEncoding<F: Field> {
    pub val: Polynomial<F>,
    pub row: Polynomial<F>,
    pub col: Polynomial<F>,
    
//    pub val_vec: Vec<F>,
//    pub val_row: Vec<F>,
//    pub val_col: Vec<F>,
    
}

impl<F: Field> Matrix<F>{
    fn max_domain(&self) -> usize {
        let mut non_zero_elements = 0;
        for row in &self.rows{
            non_zero_elements += row.iter().filter(|&n| *n != F::zero()).count();
        }
        non_zero_elements
    }

}

impl<F: Field + FftField> TryFrom<Matrix<F>> for MatrixSparseEncoding<F>{
    type Error = &'static str;
    /// Try to construct a MatrixSparseEncoding from a Vec<Vec<F>>
    fn try_from(matrix: Matrix<F>) -> Result<Self, Self::Error> {
        // build a domain of size N
        let h_size = matrix.max_domain();
        let d = GeneralEvaluationDomain::<F>::new(h_size).unwrap();
        // create 3 vectors with the cols/rows/vals of nonzero entries
        let mut rows_indexes = Vec::new();
        let mut cols_indexes = Vec::new();
        let mut vals = Vec::new();
        
        for n_row in 0..matrix.rows.len() {
            for n_col in 0..matrix.rows[n_row].len() {
                let val = matrix.rows[n_row][n_col];
                if  val != F::zero() {
                    rows_indexes.push(F::from(n_row as u32));
                    cols_indexes.push(F::from(n_col as u32));
                    vals.push(val);
                }
            }
        }
        // interpolate them
        if vals.len() > 0 {
            Ok(Self {
                row: Evaluations::from_vec_and_domain(rows_indexes, d).interpolate(),
                col: Evaluations::from_vec_and_domain(cols_indexes, d).interpolate(),
                val: Evaluations::from_vec_and_domain(vals, d).interpolate(),
            })
        }
        else {
            Err("Empty Matrix")
        }
    }
}

pub mod encoder;
pub use self::encoder::*;

pub mod prover;
pub use self::prover::*;

pub mod verifier;
pub use self::verifier::*;

// Offline phase
pub struct PhP<F: Field>{
    _field: PhantomData<F>,
}

#[cfg(test)]
mod test;
