use burn::module::Module;

#[derive(Module, Debug, Clone)]
pub struct MambaConfig {
  /// Model dimension
  pub d_model: usize,
  /// Vocabulary size
  pub vocab_size: usize,
  /// Number of layers
  pub n_layers: usize,
  /// Expansion factor
  pub expand_factor: usize,
  /// State dimension for SSM
  pub d_state: usize,
  /// Convolution kernel size
  pub d_conv: usize,
  /// Dropout rate
  pub dropout: f64,
  /// Bias in linear layers
  pub bias: bool,
}

impl Default for MambaConfig {
  fn default() -> Self {
    Self {
      d_model: 768,
      vocab_size: 50257, // Default GPT-2 vocabulary size
      n_layers: 2,
      expand_factor: 2,
      d_state: 16,
      d_conv: 4,
      dropout: 0.0,
      bias: false,
    }
  }
}
