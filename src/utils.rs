pub fn dot_product(v1: [f32; 3], v2: [f32; 3]) -> f32 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

pub fn modulus(v: [f32; 3]) -> f32{
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub fn multiply_v_3d_by_s(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

pub fn add_v3d_to_v3d(v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

pub fn flip_v3d(v: [f32; 3]) -> [f32; 3] {
    [-v[0], -v[1], -v[2]]
}

pub fn direction(v: [f32; 3]) -> [f32; 3] {
    let m = modulus(v);
    if m == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / m, v[1] / m, v[2] / m]
    }
}

pub fn from_i32_to_f32(pos: [i32; 3]) -> [f32; 3]{
    [pos[0] as f32, pos[1] as f32, pos[2] as f32]
}