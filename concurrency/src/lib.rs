mod matrix;
pub mod metrics;
mod vector;

pub use matrix::{multiply, Matrix};
pub use metrics::*;
pub use vector::{dot_product, Vector};
