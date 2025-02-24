use burn::tensor::{Tensor, TensorData};
use burn_ndarray::{NdArray, NdArrayDevice};

pub struct LinearModel {
    weight: Tensor<NdArray, 1>,
    bias: Tensor<NdArray, 1>,
}

impl LinearModel {
    pub fn new(device: &NdArrayDevice) -> Self { // Use NdArrayDevice instead of NdArray
        Self {
            weight: Tensor::from_data(TensorData::from([2.0]), device),
            bias: Tensor::from_data(TensorData::from([1.0]), device),
        }
    }

    pub fn predict(&self, x: Tensor<NdArray, 1>) -> Tensor<NdArray, 1> {
        self.weight.clone() * x + self.bias.clone()
    }
}
