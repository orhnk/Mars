use std::cmp::PartialEq;
/// # Log Loss
/// This algorithm as known as log loss allows you to detect error changes in binary classification
/// problems. It is pretty easy to understand and educational alogorithm. (Binary Cross Entropy
/// [Log Loss])

pub fn log_loss<T>(classes: Vec<T>, act: Vec<f64>, pred: Vec<f64>) -> f64
where
    T: PartialEq<T> + Copy,
{
    let mut sum = 0_f64;
    let first = &classes[0];

    for ((&act, &pred), &class) in act.iter().zip(&pred).zip(&classes) {
        sum += binary_cross_entropy(if &class == first { true } else { false }, act, pred);
    }
    sum / pred.len() as f64
}
pub fn binary_cross_entropy(boolean: bool, act: f64, pred: f64) -> f64 {
    // let class = match boolean {
    //     true => 0,
    //     _ => 1,
    // };
    let y_pred = (act - pred).abs();
    -(if boolean {
        y_pred.log(2.)
    } else {
        (1_f64 - y_pred).log(2.)
    }) // -> This function computes
       // -(class as f64 * y_pred.ln() + (1 - class) as f64 * (1_f64 - y_pred).ln()) // -> This function computes
       // log loss in a smart way which depends on class's value. look at the equation when its 0 or 1
} // I have copied all this code from chatGPT openai deep learning model! WOW !!!
  // I substract else block from 1, bcuz I want to make all inputs in the same class probability only
  // 1 or 0.

fn main() {
    panic!("This file is still in development! It is base version for `loss_functions/src/binary_cross_entropy.rs`");
    let act = vec![0.1, 0.9, 0.8, 0.35]; // These all must be in the same order to be able to
                                         // create a relation between!
    let pred = vec![0.9, 0.1, 0.2, 0.65];
    let class = vec!["Dog", "Cat", "Cat", "Dog"];

    let result = log_loss(class, act, pred);
    print!("{result}");
}
