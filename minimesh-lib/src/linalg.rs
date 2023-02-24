use std::f64::consts::PI;

use nalgebra::{Matrix3, Vector3, Translation3, Rotation3, Scale3};



// ===========================================================
// ========================= Vectors =========================
// ===========================================================


#[derive(Debug)]
pub struct PointTransfom {
    pub scaling : Scale3<f64>,
    pub translation : Translation3<f64>,
    pub rotation : Rotation3<f64>
}

impl PointTransfom {
    pub fn new() -> Self {
        Self {
            scaling : Scale3::new(1.0,1.0,1.0),
            translation : Translation3::new(0.0,0.0,0.0),
            rotation : Rotation3::from_euler_angles(0.0,0.0,0.0)
        }
    }
    pub fn rotation_radians(rx : f64, ry : f64, rz : f64) -> Self {
        Self {
            scaling : Scale3::new(1.0,1.0,1.0),
            translation : Translation3::new(0.0,0.0,0.0),
            rotation : Rotation3::from_euler_angles(rx, ry, rz)
        }
    }
    pub fn rotation_degrees(rx : f64, ry : f64, rz : f64) -> Self {
        Self {
            scaling : Scale3::new(1.0,1.0,1.0),
            translation : Translation3::new(0.0,0.0,0.0),
            rotation : Rotation3::from_euler_angles(rx * 180.0 / PI, ry * 180.0 / PI, rz * 180.0 / PI)
        }
    }
    pub fn scaling(sx : f64, sy : f64, sz : f64) -> Self {
        Self {
            scaling : Scale3::new(sx, sy, sz),
            translation : Translation3::new(0.0,0.0,0.0),
            rotation : Rotation3::from_euler_angles(0.0,0.0,0.0)
        }
    }

    pub fn translation(tx : f64, ty : f64, tz : f64) -> Self {
        Self {
            scaling : Scale3::new(1.0,1.0,1.0),
            translation : Translation3::new(tx, ty, tz),
            rotation : Rotation3::from_euler_angles(0.0,0.0,0.0)
        }
    }
}

pub fn rotation_matrix(_v1: &Vector3<f64>, v2: &Vector3<f64>) -> Matrix3<f64> {
    // rotation matrix to rotate v1 to v2
    // assumes v1 and v2 are normalized
    let unit_z = Vector3::new(0.0, 0.0, 1.0);
    
    let u3 = v2;
    let u1 = u3.cross(&unit_z);
    let u2 = u3.cross(&u1);

    Matrix3::new(u1.x,u2.x,u3.x,u1.y,u2.y,u3.y,u1.z,u2.z,u3.z)

}

pub fn array_to_matrix(arr:&[f32; 3]) -> Vector3<f64> {
    let temp = arr.map(|v| v as f64);

    Vector3::new(temp[0], temp[1], temp[2])
}