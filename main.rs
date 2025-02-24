mod model;
use model::LinearModel;
use burn::tensor::{Tensor, TensorData};
use burn_ndarray::{NdArray, NdArrayDevice};
use rand::Rng;
use textplots::{Chart, Plot, Shape};
use rand::rng; // Import new rng method

/*fn generate_data(samples: usize) -> Vec<(f32, f32)> {
    let mut rng = rand::thread_rng(); // Use thread_rng() instead of rng()
    let mut data = Vec::new();

    for _ in 0..samples {
        let x: f32 = rng.gen_range(0.0..10.0); // Use gen_range() correctly
        let noise: f32 = rng.gen_range(-1.0..1.0); // Use gen_range() correctly
        let y = 2.0 * x + 1.0 + noise;
        data.push((x, y));
    }

    data
}
*/

fn generate_data(samples: usize) -> Vec<(f32, f32)> {
    let mut rng = rand::rng(); // Replace thread_rng() with rng()

    let mut data = Vec::new();
    for _ in 0..samples {
        let x: f32 = rng.random_range(0.0..10.0); // Use random_range instead of gen_range
        let noise: f32 = rng.random_range(-1.0..1.0);
        let y = 2.0 * x + 1.0 + noise;
        data.push((x, y));
    }
    data
}

fn main() {
    let device = NdArrayDevice::default(); // Ensure correct device usage
    let model = LinearModel::new(&device);

    // Example input tensor
    let x = Tensor::<NdArray, 1>::from_data(TensorData::from([5.0]), &device);
    let y_pred = model.predict(x);

    println!("Prediction: {:?}", y_pred.to_data());

    // Plot the data using textplots
    let data = generate_data(10); // Generate sample data
    println!("Linear Regression Model Output:");
    Chart::new(100, 30, 0.0, 10.0)
        .lineplot(&Shape::Lines(&data))
        .display();
}
