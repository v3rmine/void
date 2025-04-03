use burn::{
  backend::{NdArray, ndarray::NdArrayDevice},
  tensor::{Int, Tensor},
};
use mamba_burn::{config::MambaConfig, model::SimpleMamba};

// Define type aliases to simplify
type TestBackend = NdArray<f32>;
type TestDevice = NdArrayDevice;

#[test]
fn test_simple_mamba_forward() {
  // Initialize the device
  let device = TestDevice::default();

  // Create a test configuration
  let config = MambaConfig {
    d_model: 32,
    vocab_size: 100,
    n_layers: 1,
    expand_factor: 2,
    d_state: 4,
    d_conv: 2,
    dropout: 0.0,
    bias: true,
  };

  // Initialize the model
  let model = SimpleMamba::<TestBackend>::new(&device, config);

  // Create a test input batch
  let batch_size = 2;
  let seq_len = 10;
  let input = Tensor::<TestBackend, 2, Int>::ones([batch_size, seq_len], &device);

  // Pass the input through the model
  let output = model.forward(input);

  // Verify the output dimensions
  assert_eq!(output.dims(), [batch_size, seq_len, 100]);
}
