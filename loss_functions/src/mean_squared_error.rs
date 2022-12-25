pub fn mean_squared_error(act: &Vec<f64>, pred: &Vec<f64>) -> f64 {
    let mut sum = 0.0; // -> initialize sum of every element
    for (act, pred) in act.iter().zip(pred) {
        sum += (act - pred).abs().powi(2);
    }
    sum / pred.len() as f64
}
