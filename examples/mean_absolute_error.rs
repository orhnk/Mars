mod base;

fn mean_abs_error(actual: &Vec<f64>, predictions: &Vec<f64>) -> f64 {
    /* This function take two paramaters:
     * actual goal and prediction that model
     * has made, enter it wisely! :D     */
    let mut result = Vec::new();
    for (pred, act) in predictions.iter().zip(actual) {
        let diff = (pred - act).abs();
        result.push(diff);
    }
    base::mean(result)
}

fn main() {
    let act = vec![1.1, 2.0, 1.7];
    let pred = vec![1.0, 1.7, 1.5];
    let result = mean_abs_error(&act, &pred);
    print!("{result}");
}
