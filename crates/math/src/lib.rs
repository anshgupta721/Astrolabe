use nalgebra::{SVector, UnitQuaternion, SMatrix, Rotation3};

pub type Vec3 = SVector<f64, 3>;
pub type Mat3 = SMatrix<f64, 3, 3>;
pub type Quaternion = UnitQuaternion<f64>;
pub type Rot3 = Rotation3<f64>;