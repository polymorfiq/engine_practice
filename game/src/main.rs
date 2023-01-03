use linalg::matrix;

fn main() {
    let a = matrix([
        [1, 2, 3],
        [4, 5, 6]
    ]);

    let b = matrix([
        [8, 9],
        [10, 11],
        [12, 13],
    ]);

    let c = a.dot(&b);
    for row in 0..c.rows() {
        println!("c, Row {}: {:?}!", row, c.row(row).raw());
    }
}
