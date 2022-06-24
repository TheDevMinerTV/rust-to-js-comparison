pub type Matrix4 = [[f64; 4]; 4];

pub fn new_matrix4() -> Matrix4 {
    return [
        [0f64, 0f64, 0f64, 0f64],
        [0f64, 0f64, 0f64, 0f64],
        [0f64, 0f64, 0f64, 0f64],
        [0f64, 0f64, 0f64, 0f64],
    ];
}

pub fn mult_mat4(m1: &Matrix4, m2: &Matrix4) -> Matrix4 {
    let mut m3 = new_matrix4();

    for x in 0..4 {
        for y in 0..4 {
            m3[x][y] = m1[x][y] * m2[x][y];
        }
    }

    m3
}
