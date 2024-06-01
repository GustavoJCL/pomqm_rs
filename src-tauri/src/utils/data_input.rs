use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OptimizationType {
    Maximization,
    Minimization,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ComparatorType {
    Equal,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataInput {
    pub optimization: OptimizationType,
    pub xi: Vec<Vec<f64>>,
    pub comparator_type: Vec<ComparatorType>,
    pub rhs: Vec<f64>,
}
