use linalg::Matrix;

fn main() {
    let a = Matrix::new([
        [1, 2, 3],
        [4, 5, 6]
    ]);

    let b = Matrix::new([
        [8, 9],
        [10, 11],
        [12, 13],
    ]);

    let c = a.dot(&b);
    for row in 0..c.rows() {
        println!("c, Row {}: {:?}!", row, c.row(row));
    }
}
