use thiserror::Error;

#[derive(Error, Debug)]
pub enum KfError {
    #[error("Innovation matrix is singular")]
    SingularInnovationMatrix,
}
