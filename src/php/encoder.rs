use algebra_core::{Field};
use ff_fft::{DensePolynomial as Polynomial};
use crate::php::MatrixSparseEncoding;

pub struct RelationEncoder<F: Field>{
    pub L: MatrixSparseEncoding<F>,
    pub R: MatrixSparseEncoding<F>,
}


