use matrix;

use crate::transform;



pub struct AffineCamera {
    pub affine: matrix::format::Conventional<f32>
}

impl AffineCamera {
    pub fn testfunc2(self){
        println!("get func'd");
    }
}


pub fn testfunc(){
    let t = transform::Vec3 { x: 0.0, y: 0.0, z: 0.0 };
}