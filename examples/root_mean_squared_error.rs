pub fn root_mean_squared_error(act: &Vec<f64>, pred: &Vec<f64>) -> f64 {
    let mut result = 0_f64;
    for (act, pred) in act.iter().zip(pred) {
        result += squared_err(*act, *pred);
    }
    (result / act.len() as f64).sqrt()
}
fn squared_err(act: f64, pred: f64) -> f64 {
    let diff = (act - pred).abs();
    diff * diff
}

fn main() {
    let act = vec![1.1, 2., 1.7];
    let pred = vec![1., 1.7, 1.5];
    let result = root_mean_squared_error(&act, &pred);
    println!("{result}")
}
