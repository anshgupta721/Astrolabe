use nalgebra::SVector;

use super::Kf;
use crate::kalman_filters::kf_error::KfError;
// TODO: KalmanDiagnostics for ensuring the filter is behaving correctly

pub struct KalmanFilter<const N: usize, const M: usize> {
    // state array
    // measurements
    // other stuff
}

impl<const N: usize, const M: usize> Kf<N, M> for KalmanFilter<N, M> {
    fn predict(&mut self, dt: f64) {}

    fn update(&mut self, z: &SVector<f64, M>) {}

    fn get_state(&self) -> Result<SVector<f64, N>, KfError> {
        return Err(KfError::SingularInnovationMatrix);
    }
}
