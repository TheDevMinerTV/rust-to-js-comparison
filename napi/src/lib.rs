#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

#[napi]
fn empty_fn() {}

#[napi]
fn mult_mat4(env: Env, arr1: Array, arr2: Array) -> Array {
    let mut m1 = logic::new_matrix4();
    let mut m2 = logic::new_matrix4();

    for i in 0..4u32 {
        let r1 = arr1.get::<Array>(i).unwrap().unwrap();
        let r2 = arr2.get::<Array>(i).unwrap().unwrap();

        for j in 0..4u32 {
            m1[i as usize][j as usize] = r1.get::<f64>(j).unwrap().unwrap();
            m2[i as usize][j as usize] = r2.get::<f64>(j).unwrap().unwrap();
        }
    }

    let m3 = logic::mult_mat4(&m1, &m2);

    let mut arr3 = env.create_array(4).unwrap();
    for x in 0u32..4u32 {
        arr3.set(x, Array::from_ref_vec(&env, &m3[x as usize]).unwrap())
            .unwrap();
    }

    arr3
}
