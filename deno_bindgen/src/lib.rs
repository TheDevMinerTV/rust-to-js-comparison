use deno_bindgen::deno_bindgen;

#[deno_bindgen]
pub fn empty_fn() {}

// We can't yet use the `#[deno_bindgen]` attribute for `Vec<T>` so we have to make a wrapper struct
#[deno_bindgen]
pub struct Matrix4 {
    pub d: Vec<Vec<f64>>,
}

#[deno_bindgen]
pub fn mult_mat4(arr1: Matrix4, arr2: Matrix4) -> Matrix4 {
    let mut m1 = logic::new_matrix4();
    let mut m2 = logic::new_matrix4();

    for x in 0..4 {
        for y in 0..4 {
            m1[x][y] = arr1.d[x][y];
            m2[x][y] = arr2.d[x][y];
        }
    }

    let m3 = logic::mult_mat4(&m1, &m2);

    Matrix4 {
        d: m3.map(|r| r.to_vec()).to_vec(),
    }
}
