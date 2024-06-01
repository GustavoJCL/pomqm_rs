use crate::utils::data_input::DataInput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SimplexDualDataOutput {
    basic_variables: Vec<String>,
    z: Vec<f64>,
    xi: Vec<Vec<f64>>,
    hi: Vec<Vec<f64>>,
    solution_factor: Vec<f64>,
}

impl SimplexDualDataOutput {
    fn new(
        basic_variables: Vec<String>,
        z: Vec<f64>,
        xi: Vec<Vec<f64>>,
        hi: Vec<Vec<f64>>,
        solution_factor: Vec<f64>,
    ) -> Self {
        Self {
            basic_variables,
            z,
            xi,
            hi,
            solution_factor,
        }
    }
}
#[tauri::command(rename_all = "snake_case")]
pub fn simplex_dual(data_input: &DataInput) -> Vec<SimplexDualDataOutput> {}
