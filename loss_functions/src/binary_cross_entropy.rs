
/// # Log Loss
/// This algorithm as known as log loss allows you to detect error changes in binary classification
/// problems. It is pretty easy to understand and educational alogorithm. (Binary Cross Entropy
/// [Log Loss])

pub fn binary_cross_entropy_loss(y: f32, y_pred: f32) -> f32 {
    -(y * y_pred.max(1e-7).ln() + (1.0 - y) * (1.0 - y_pred.max(1e-7)).ln())
} // I have copied all this code from chatGPT openai deep learning model! WOW !!!
