use algebra::{
    bls12_381::{Fr},
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use algebra_core::{
    UniformRand,
};

use crate::php::Matrix;
use crate::php::MatrixSparseEncoding;
use std::convert::TryFrom;

#[test]
fn try_from_matrix2x2(){
    let mut rng = rand::thread_rng();
    let mut row1 = Vec::new();
    let mut row2 = Vec::new();
    for _i in 0..2 {
     row1.push(Fr::rand(&mut rng));
     row2.push(Fr::rand(&mut rng));
    }
    let matrix = Matrix {
        rows: (&[row1, row2]).to_vec(),
    };
    let matrix_encoding = MatrixSparseEncoding::try_from(matrix.clone()).unwrap();
    
    for j in 0..4 {
        let j_Fr = Fr::from(j as u32);
        let row_j = matrix_encoding.row.evaluate(j_Fr);
        let col_j = matrix_encoding.col.evaluate(j_Fr);
        let val_j = matrix_encoding.val.evaluate(j_Fr);
        println!("row={}, col={}, val={} ", row_j, col_j, val_j);
        // this should give 
        // 1,1,v1
        // 1,2,v2
        // 2,1,v3
        // 2,2,v4
    }
}






