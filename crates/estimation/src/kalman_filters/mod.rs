use nalgebra::SVector;
use crate::kalman_filters::kf_error::KfError;

pub mod kf;
pub mod kf_error;

pub trait Kf<const N: usize, const M: usize> {
    fn predict(&mut self, dt: f64); // Step dynamics forward
    fn update(&mut self, z: &SVector<f64,M>); // Update based on measurements
    fn get_state(&self) -> Result<SVector<f64, N>, KfError>; //getter function for receiving kf state
}