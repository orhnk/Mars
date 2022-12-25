extern crate csv;
extern crate gnuplot;

use gnuplot::{
    Figure,
    Color,
}
;
use std::io;
use std::thread;

// fn main() {
//     let mut x: Vec<f64> = Vec::new();
//     let mut y: Vec<f64> = Vec::new();
//     let mut fg = Figure::new();
//     let axes = fg.axes2d();
//     let mut rdr =
//         csv::Reader::from_path("/home/kobruh/github/mars/examples/example_data/test.csv").unwrap();
//     for result in rdr.records() {
//         let record = result.unwrap();
//         x.push(record[0].parse().unwrap());
//         y.push(record[1].parse().unwrap());
//         axes.points(
//             &x, // x-coordinates
//             &y, // y-coordinates
//             &[],
//         );
//     }

//     fg.show().unwrap();
// }

/* This is another section which has been created by an AI and modified by a human-like KoBruhh */

struct DataPoint {
    x: f64,
    y: f64,
}

impl DataPoint {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Default for DataPoint {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

struct Model {
    w: f64,
    b: f64,
}

impl Model {
    fn new(w: f64, b: f64) -> Self {
        Self { w, b }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self { w: 0.0, b: 0.0 }
    }
}

impl Model {
    /// Returns the predicted y value for a given x value based on the model's slope and intercept.
    fn predict(&self, x: f64) -> f64 {
        self.w * x + self.b
    }

    /// Calculates the mean squared error loss for a given set of data points.
    fn loss(&self, data: &[DataPoint]) -> f64 {
        let mut loss = 0.0;
        for point in data {
            let error = self.predict(point.x) - point.y;
            loss += error * error;
        }
        loss / data.len() as f64
    }

    /// Trains the model using the gradient descent algorithm.
    fn train(&mut self, data: &[DataPoint], learning_rate: f64, num_iterations: usize) {
        for _ in 0..num_iterations {
            // Calculate the gradient of the loss function with respect to the model's parameters.
            let mut dw = 0.0;
            let mut db = 0.0;
            for point in data {
                let error = self.predict(point.x) - point.y;
                dw += error * point.x;
                db += error;
            }
            // Update the model's parameters based on the gradient and the learning rate.
            self.w -= learning_rate * dw / data.len() as f64;
            self.b -= learning_rate * db / data.len() as f64;
        }
    }
}

fn plot(mut fg: Figure, x: &Vec<f64>, y: &Vec<f64>, xs:&Vec<f64>, ys:&Vec<f64>) {
    let axes = fg.axes2d();
    axes.points(
        x, // x-coordinates
        y, // y-coordinates
        &[Color("green")],
    )
    .lines(xs, ys, &[Color("red")]);
    fg.show()
        .expect("Couldn't plot the Figure you gave (with the dataset)");
}

pub fn run() {
    eprintln!("This neural network is trained to take y = 5x + 13 -> sample linear equation");
    let learning_rate = 0.0001;
    let num_iterations = 10000000;

    let fg = Figure::new(); //-| For plotting
    let mut x = Vec::new(); //----| ^^^^^^^^^^^^
    let mut y = Vec::new(); //----|
    let mut train_data:Vec<DataPoint> = Vec::new();

    // let record:Vec<Vec<f64>>;
    let mut rdr = csv::Reader::from_path("example_data/test.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        // for point in &train_data {
        // x.push(point.x);
        // y.push(point.y);
        x.push(record[0].parse().unwrap());
        y.push(record[1].parse().unwrap());
    }
    let mut model: Model = Default::default(); // Model changes it's weights and biases by the time
    // passes. That's why It has to be marked as mutable.
    for (x, y) in x.iter().zip(&y) {
        train_data.push(DataPoint::new(*x, *y));
    }

    model.train(&train_data, learning_rate, num_iterations);
    let loss = model.loss(&train_data);
    let pred = model.predict(10.);
    println!("Loss: {loss}, Pred 10.0: {pred}");
    println!("please enter some text to send inputs to your model");

    // let (max, min) = (record[0].iter().max(), record[1].iter().min());
    let xs = vec![-10.0, -1.0, 0.0, 2.0, 12.0, 34.0, 55.0, 100.];
    let ys: Vec<f64> = xs.iter().map(|x| model.predict(*x)).collect();

    // let equation = model.w + model.b; // -> (x, y) values for linear line equation
    thread::spawn(move || {
        plot(fg, &x, &y, &xs, &ys);
    });

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let num: f64 = input.trim().parse().unwrap();
        let pred = model.predict(num);
        println!("Model Predicted: {pred}");
    }
}
/* This is for y = 5x + 13 */
/* This is a gradient descent implementation so it can find any y = ax + b function! wow */
