/* This code is fully written by an AI called ChatGPT It is not tested or comprehended for now :( */


use std::collections::VecDeque;

// Decision tree node structure
struct DecisionTreeNode {
    // Split point for continuous features
    split_point: Option<f64>,
    // Feature index to split on
    feature_index: usize,
    // Value for categorical features or for leaf nodes
    value: f64,
    // Children nodes
    left: Option<Box<DecisionTreeNode>>,
    right: Option<Box<DecisionTreeNode>>,
}

// Decision tree structure
struct DecisionTree {
    root: DecisionTreeNode,
}

// Gradient boosting model structure
struct GradientBoostingModel {
    // Base model (decision tree)
    base_model: DecisionTree,
    // Weights of the base models
    weights: Vec<f64>,
    // Shrinkage factor
    shrinkage: f64,
}

impl GradientBoostingModel {
    fn new(shrinkage: f64) -> Self {
        Self {
            base_model: DecisionTree::new(),
            weights: Vec::new(),
            shrinkage,
        }
    }

    fn fit(&mut self, X: &[Vec<f64>], y: &[f64]) {
        let n_samples = X.len();
        let mut residuals = y.to_vec();

        // Fit base models and update weights
        for _ in 0..self.n_estimators {
            self.base_model.fit(&X, &residuals);
            let predictions = self.base_model.predict(X);
            let mut gradient = Vec::with_capacity(n_samples);
            for i in 0..n_samples {
                gradient.push(residuals[i] / (1.0 + predictions[i]));
            }
            let weight = self.shrinkage * (1.0 / (1.0 + self.base_model.predict(&gradient)[0]));
            self.weights.push(weight);
            for i in 0..n_samples {
                residuals[i] -= weight * predictions[i];
            }
        }
    }

    fn predict(&self, X: &[Vec<f64>]) -> Vec<f64> {
        let mut predictions = Vec::with_capacity(X.len());
        for x in X {
            let mut prediction = 0.0;
            let mut base_model_predictions = self.base_model.predict(x);
            for (weight, base_model_prediction) in self.weights.iter().zip(base_model_predictions) {
                prediction += weight * base_model_prediction;
            }
            predictions.push(prediction);
        }
        predictions
    }
}


