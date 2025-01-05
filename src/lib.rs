#[derive(Debug)]
pub struct Matrix {
    row: usize,
    col: usize,
    data: Vec<Vec<f64>>
}


impl Matrix {
    fn new(row: usize, col: usize) -> Matrix {
        Matrix {
            row: row,
            col: col,
            data: vec![vec![0.0; row]; col],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn con() {
        let m: Matrix = Matrix::new(3, 3);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(m.data[i][j], 0.0)
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                assert_ne!(m.data[i][j], 1.0)
            }
        }
    }
}
