//! Automatic regression model selection for benchmark trend analysis
//!
//! Analyzes benchmark data distributions and automatically selects the best-fitting
//! complexity model (O(1), O(log n), O(n), O(n log n), O(n²), etc.) using BIC scoring.

/// Represents different complexity models for regression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    /// Constant time: y = a
    Constant,
    /// Logarithmic: y = a + b*log(x)
    Logarithmic,
    /// Linear: y = a + b*x
    Linear,
    /// Linearithmic: y = a + b*(x*log(x))
    Linearithmic,
    /// Quadratic: y = a + b*x²
    Quadratic,
    /// Mixed quadratic: y = a + b*x + c*x²
    Mixed,
    /// Cubic: y = a + b*x + c*x² + d*x³
    Cubic,
    /// Power law: y = a + c*x^b
    PowerLaw,
}

impl ModelType {
    /// Number of parameters for this model
    fn param_count(self) -> usize {
        match self {
            ModelType::Constant => 1,
            ModelType::Logarithmic => 2,
            ModelType::Linear => 2,
            ModelType::Linearithmic => 2,
            ModelType::Quadratic => 2,
            ModelType::Mixed => 3,
            ModelType::Cubic => 4,
            ModelType::PowerLaw => 3,
        }
    }

    /// Minimum number of data points required to fit this model
    fn min_points(self) -> usize {
        match self {
            ModelType::Constant => 1,
            ModelType::Logarithmic => 2,
            ModelType::Linear => 2,
            ModelType::Linearithmic => 2,
            ModelType::Quadratic => 2,
            ModelType::Mixed => 3,
            ModelType::Cubic => 4,
            ModelType::PowerLaw => 3,
        }
    }
}

/// A fitted regression model with its coefficients
#[derive(Debug, Clone)]
pub struct SelectedModel {
    pub model_type: ModelType,
    pub coefficients: Vec<f64>,
    pub bic: f64,
}

impl SelectedModel {
    /// Predict y value for a given x
    pub fn predict(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return self.coefficients[0]; // Return intercept for invalid x
        }

        match self.model_type {
            ModelType::Constant => self.coefficients[0],
            ModelType::Logarithmic => {
                let a = self.coefficients[0];
                let b = self.coefficients[1];
                a + b * x.ln()
            }
            ModelType::Linear => {
                let a = self.coefficients[0];
                let b = self.coefficients[1];
                a + b * x
            }
            ModelType::Linearithmic => {
                let a = self.coefficients[0];
                let b = self.coefficients[1];
                a + b * x * x.ln()
            }
            ModelType::Quadratic => {
                let a = self.coefficients[0];
                let b = self.coefficients[1];
                a + b * x * x
            }
            ModelType::Mixed => {
                let a = self.coefficients[0];
                let b = self.coefficients[1];
                let c = self.coefficients[2];
                a + b * x + c * x * x
            }
            ModelType::Cubic => {
                let a = self.coefficients[0];
                let b = self.coefficients[1];
                let c = self.coefficients[2];
                let d = self.coefficients[3];
                a + b * x + c * x * x + d * x * x * x
            }
            ModelType::PowerLaw => {
                let a = self.coefficients[0];
                let c = self.coefficients[1];
                let b = self.coefficients[2];
                a + c * x.powf(b)
            }
        }
    }
}

/// Calculate BIC (Bayesian Information Criterion)
fn calculate_bic(n: usize, sse: f64, k: usize) -> f64 {
    if n == 0 || sse <= 0.0 {
        return f64::INFINITY;
    }
    let n_f = n as f64;
    let k_f = k as f64;
    n_f * (sse / n_f).ln() + k_f * n_f.ln()
}

/// Calculate sum of squared errors for a model
fn calculate_sse<F>(points: &[(f64, f64)], predict: F) -> f64
where
    F: Fn(f64) -> f64,
{
    points
        .iter()
        .map(|&(x, y)| {
            let pred = predict(x);
            let diff = y - pred;
            diff * diff
        })
        .sum()
}

/// Fit constant model: y = a
fn fit_constant(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.is_empty() {
        return None;
    }
    let mean_y = points.iter().map(|&(_, y)| y).sum::<f64>() / points.len() as f64;
    Some(vec![mean_y])
}

/// Fit logarithmic model: y = a + b*log(x)
fn fit_logarithmic(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.len() < 2 {
        return None;
    }

    let mut sum_1 = 0.0;
    let mut sum_logx = 0.0;
    let mut sum_logx2 = 0.0;
    let mut sum_y = 0.0;
    let mut sum_logxy = 0.0;

    for &(x, y) in points {
        if x <= 0.0 {
            return None;
        }
        let logx = x.ln();
        sum_1 += 1.0;
        sum_logx += logx;
        sum_logx2 += logx * logx;
        sum_y += y;
        sum_logxy += logx * y;
    }

    let denom = sum_1 * sum_logx2 - sum_logx * sum_logx;
    if denom.abs() < 1e-20 {
        return None;
    }

    let b = (sum_1 * sum_logxy - sum_logx * sum_y) / denom;
    let a = (sum_y - b * sum_logx) / sum_1;

    Some(vec![a, b])
}

/// Fit linear model: y = a + b*x
fn fit_linear(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.len() < 2 {
        return None;
    }

    let n = points.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;

    for &(x, y) in points {
        sum_x += x;
        sum_x2 += x * x;
        sum_y += y;
        sum_xy += x * y;
    }

    let denom = n * sum_x2 - sum_x * sum_x;
    if denom.abs() < 1e-20 {
        return None;
    }

    let b = (n * sum_xy - sum_x * sum_y) / denom;
    let a = (sum_y - b * sum_x) / n;

    Some(vec![a, b])
}

/// Fit linearithmic model: y = a + b*(x*log(x))
fn fit_linearithmic(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.len() < 2 {
        return None;
    }

    let mut sum_1 = 0.0;
    let mut sum_xlogx = 0.0;
    let mut sum_xlogx2 = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xlogxy = 0.0;

    for &(x, y) in points {
        if x <= 0.0 {
            return None;
        }
        let xlogx = x * x.ln();
        sum_1 += 1.0;
        sum_xlogx += xlogx;
        sum_xlogx2 += xlogx * xlogx;
        sum_y += y;
        sum_xlogxy += xlogx * y;
    }

    let denom = sum_1 * sum_xlogx2 - sum_xlogx * sum_xlogx;
    if denom.abs() < 1e-20 {
        return None;
    }

    let b = (sum_1 * sum_xlogxy - sum_xlogx * sum_y) / denom;
    let a = (sum_y - b * sum_xlogx) / sum_1;

    Some(vec![a, b])
}

/// Fit quadratic model: y = a + b*x²
fn fit_quadratic(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.len() < 2 {
        return None;
    }

    let mut sum_1 = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_x4 = 0.0;
    let mut sum_y = 0.0;
    let mut sum_x2y = 0.0;

    for &(x, y) in points {
        let x2 = x * x;
        sum_1 += 1.0;
        sum_x2 += x2;
        sum_x4 += x2 * x2;
        sum_y += y;
        sum_x2y += x2 * y;
    }

    let denom = sum_1 * sum_x4 - sum_x2 * sum_x2;
    if denom.abs() < 1e-20 {
        return None;
    }

    let b = (sum_1 * sum_x2y - sum_x2 * sum_y) / denom;
    let a = (sum_y - b * sum_x2) / sum_1;

    Some(vec![a, b])
}

/// Fit mixed quadratic model: y = a + b*x + c*x²
fn fit_mixed(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.len() < 3 {
        return None;
    }

    let mut sum_1 = 0.0;
    let mut sum_x = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_x3 = 0.0;
    let mut sum_x4 = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_x2y = 0.0;

    for &(x, y) in points {
        let x2 = x * x;
        let x3 = x2 * x;
        let x4 = x3 * x;
        sum_1 += 1.0;
        sum_x += x;
        sum_x2 += x2;
        sum_x3 += x3;
        sum_x4 += x4;
        sum_y += y;
        sum_xy += x * y;
        sum_x2y += x2 * y;
    }

    // Solve 3x3 system using Gaussian elimination
    let mut mat = [[sum_1, sum_x, sum_x2], [sum_x, sum_x2, sum_x3], [sum_x2, sum_x3, sum_x4]];
    let mut rhs = [sum_y, sum_xy, sum_x2y];

    // Gaussian elimination with partial pivoting
    for i in 0..3 {
        let mut max_row = i;
        let mut max_val = mat[i][i].abs();
        for k in (i + 1)..3 {
            if mat[k][i].abs() > max_val {
                max_val = mat[k][i].abs();
                max_row = k;
            }
        }

        if max_row != i {
            mat.swap(i, max_row);
            rhs.swap(i, max_row);
        }

        let pivot = mat[i][i];
        if pivot.abs() < 1e-10 {
            return None;
        }

        for k in (i + 1)..3 {
            let factor = mat[k][i] / pivot;
            for j in i..3 {
                mat[k][j] -= factor * mat[i][j];
            }
            rhs[k] -= factor * rhs[i];
        }
    }

    // Back substitution
    let mut coeffs = [0.0; 3];
    for i in (0..3).rev() {
        let mut sum = rhs[i];
        for j in (i + 1)..3 {
            sum -= mat[i][j] * coeffs[j];
        }
        coeffs[i] = sum / mat[i][i];
    }

    Some(vec![coeffs[0], coeffs[1], coeffs[2]])
}

/// Fit cubic model: y = a + b*x + c*x² + d*x³
fn fit_cubic(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.len() < 4 {
        return None;
    }

    let mut sum_1 = 0.0;
    let mut sum_x = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_x3 = 0.0;
    let mut sum_x4 = 0.0;
    let mut sum_x5 = 0.0;
    let mut sum_x6 = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_x2y = 0.0;
    let mut sum_x3y = 0.0;

    for &(x, y) in points {
        let x2 = x * x;
        let x3 = x2 * x;
        let x4 = x3 * x;
        let x5 = x4 * x;
        let x6 = x5 * x;

        sum_1 += 1.0;
        sum_x += x;
        sum_x2 += x2;
        sum_x3 += x3;
        sum_x4 += x4;
        sum_x5 += x5;
        sum_x6 += x6;
        sum_y += y;
        sum_xy += x * y;
        sum_x2y += x2 * y;
        sum_x3y += x3 * y;
    }

    // Solve 4x4 system using Gaussian elimination
    let mut mat = [
        [sum_1, sum_x, sum_x2, sum_x3],
        [sum_x, sum_x2, sum_x3, sum_x4],
        [sum_x2, sum_x3, sum_x4, sum_x5],
        [sum_x3, sum_x4, sum_x5, sum_x6],
    ];
    let mut rhs = [sum_y, sum_xy, sum_x2y, sum_x3y];

    // Gaussian elimination with partial pivoting
    for i in 0..4 {
        let mut max_row = i;
        let mut max_val = mat[i][i].abs();
        for k in (i + 1)..4 {
            if mat[k][i].abs() > max_val {
                max_val = mat[k][i].abs();
                max_row = k;
            }
        }

        if max_row != i {
            mat.swap(i, max_row);
            rhs.swap(i, max_row);
        }

        let pivot = mat[i][i];
        if pivot.abs() < 1e-10 {
            return None;
        }

        for k in (i + 1)..4 {
            let factor = mat[k][i] / pivot;
            for j in i..4 {
                mat[k][j] -= factor * mat[i][j];
            }
            rhs[k] -= factor * rhs[i];
        }
    }

    // Back substitution
    let mut coeffs = [0.0; 4];
    for i in (0..4).rev() {
        let mut sum = rhs[i];
        for j in (i + 1)..4 {
            sum -= mat[i][j] * coeffs[j];
        }
        coeffs[i] = sum / mat[i][i];
    }

    Some(vec![coeffs[0], coeffs[1], coeffs[2], coeffs[3]])
}

/// Fit power law model: y = a + c*x^b
fn fit_power_law(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.len() < 3 {
        return None;
    }

    // Find minimum y value
    let min_y = points.iter().map(|&(_, y)| y).fold(f64::INFINITY, f64::min);
    if min_y <= 0.0 {
        return None;
    }

    // Try a few values of 'a' around min_y and pick the best fit
    let mut best_sse = f64::INFINITY;
    let mut best_coeffs = None;

    for a_guess in [min_y * 0.95, min_y * 0.99, min_y * 0.999] {
        if a_guess >= min_y {
            continue;
        }

        // Transform: log(y - a) = log(c) + b*log(x)
        let mut valid_points = Vec::new();
        let mut all_valid = true;
        for &(x, y) in points {
            if x <= 0.0 || y <= a_guess {
                all_valid = false;
                break;
            }
            let log_y_minus_a = (y - a_guess).ln();
            let log_x = x.ln();
            valid_points.push((log_x, log_y_minus_a));
        }

        if !all_valid || valid_points.len() < 2 {
            continue;
        }

        // Fit linear model: log(y-a) = log(c) + b*log(x)
        if let Some(log_coeffs) = fit_linear(&valid_points) {
            let log_c = log_coeffs[0];
            let b = log_coeffs[1];
            let c = log_c.exp();

            // Calculate SSE for this model
            let sse = calculate_sse(points, |x| {
                if x <= 0.0 {
                    return a_guess;
                }
                a_guess + c * x.powf(b)
            });

            if sse < best_sse {
                best_sse = sse;
                best_coeffs = Some(vec![a_guess, c, b]);
            }
        }
    }

    best_coeffs
}

/// Select the best regression model for the given data points
///
/// IMPORTANT: The input points should be sorted by x (input size) in ascending order.
pub fn select_best_model(points: &[(f64, f64)]) -> Option<SelectedModel> {
    if points.is_empty() {
        return None;
    }

    // Find x range for safety checks
    let x_min = points.iter().map(|&(x, _)| x).fold(f64::INFINITY, f64::min);
    let x_max = points.iter().map(|&(x, _)| x).fold(f64::NEG_INFINITY, f64::max);

    if x_min >= x_max {
        return None;
    }

    let n = points.len();
    let mut candidates = Vec::new();

    // Try each model type
    let model_types = [
        ModelType::Constant,
        ModelType::Logarithmic,
        ModelType::Linear,
        ModelType::Linearithmic,
        ModelType::Quadratic,
        ModelType::Mixed,
        ModelType::Cubic,
        ModelType::PowerLaw,
    ];

    for model_type in model_types.iter() {
        if n < model_type.min_points() {
            continue;
        }

        let coeffs_opt = match model_type {
            ModelType::Constant => fit_constant(points),
            ModelType::Logarithmic => fit_logarithmic(points),
            ModelType::Linear => fit_linear(points),
            ModelType::Linearithmic => fit_linearithmic(points),
            ModelType::Quadratic => fit_quadratic(points),
            ModelType::Mixed => fit_mixed(points),
            ModelType::Cubic => fit_cubic(points),
            ModelType::PowerLaw => fit_power_law(points),
        };

        if let Some(coeffs) = coeffs_opt {
            // Create temporary model to check predictions
            let temp_model =
                SelectedModel { model_type: *model_type, coefficients: coeffs.clone(), bic: 0.0 };

            // Safety check: predictions should be reasonable
            let y_min = points.iter().map(|&(_, y)| y).fold(f64::INFINITY, f64::min);
            let y_max = points.iter().map(|&(_, y)| y).fold(f64::NEG_INFINITY, f64::max);
            let y_range = y_max - y_min;

            // Check predictions at several points
            let check_points = vec![x_min, (x_min + x_max) / 2.0, x_max];

            let mut has_invalid_prediction = false;
            for &x_check in &check_points {
                let pred = temp_model.predict(x_check);
                let tolerance = y_range * 0.01;
                if pred < -tolerance || pred > y_max + y_range * 10.0 {
                    has_invalid_prediction = true;
                    break;
                }
            }

            if has_invalid_prediction {
                continue;
            }

            // Calculate SSE and BIC
            let sse = calculate_sse(points, |x| temp_model.predict(x));
            let k = model_type.param_count();
            let bic = calculate_bic(n, sse, k);

            candidates.push(SelectedModel { model_type: *model_type, coefficients: coeffs, bic });
        }
    }

    if candidates.is_empty() {
        return None;
    }

    // Sort by BIC (lower is better)
    candidates.sort_by(|a, b| a.bic.partial_cmp(&b.bic).unwrap_or(std::cmp::Ordering::Equal));

    // Prefer simpler models if BIC is close (within 2 units)
    if candidates.len() >= 2 {
        let best_bic = candidates[0].bic;
        let best_params = candidates[0].model_type.param_count();

        for candidate in candidates.iter().skip(1) {
            let candidate_params = candidate.model_type.param_count();
            if candidate_params < best_params && (candidate.bic - best_bic) < 2.0 {
                return Some(candidate.clone());
            }
            if (candidate.bic - best_bic) > 6.0 {
                break;
            }
        }
    }

    Some(candidates[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_fit() {
        // Test with linear data: y = 2x
        let points = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0), (4.0, 8.0), (5.0, 10.0)];
        let model = select_best_model(&points).unwrap();

        // The model should predict values accurately regardless of type
        // Linear or linearithmic models should both work well on this data
        let pred_3 = model.predict(3.0);
        let pred_5 = model.predict(5.0);

        // Allow some tolerance for different model types
        assert!(
            (pred_3 - 6.0).abs() < 1.0,
            "Prediction at x=3 should be close to 6, got {}",
            pred_3
        );
        assert!(
            (pred_5 - 10.0).abs() < 1.0,
            "Prediction at x=5 should be close to 10, got {}",
            pred_5
        );
    }

    #[test]
    fn test_constant_fit() {
        let points = vec![(1.0, 5.0), (2.0, 5.0), (3.0, 5.0), (4.0, 5.0)];
        let model = select_best_model(&points).unwrap();
        assert!(matches!(model.model_type, ModelType::Constant));
    }

    #[test]
    fn test_predict() {
        let model =
            SelectedModel { model_type: ModelType::Linear, coefficients: vec![0.0, 2.0], bic: 0.0 };
        assert!((model.predict(5.0) - 10.0).abs() < 0.001);
    }
}
