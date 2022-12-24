extern crate csv;
extern crate gnuplot;

use gnuplot::Figure;

fn main() {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut fg = Figure::new();
    let axes = fg.axes2d();
    let mut rdr =
        csv::Reader::from_path("/home/kobruh/github/mars/examples/example_data/test.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        x.push(record[0].parse().unwrap());
        y.push(record[1].parse().unwrap());
        axes.points(
            &x, // x-coordinates
            &y, // y-coordinates
            &[],
        );
    }

    // axes.set_color(Color::RGB(1, 0, 0)); // Set the color to red}
    fg.show().unwrap();
}
