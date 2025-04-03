use burn::prelude::*;
use nn::{Embedding, EmbeddingConfig, Linear, LinearConfig};

use crate::config::MambaConfig;

/// Minimal version of Mamba without SSM, just to test the data flow
#[derive(Module, Debug)]
pub struct SimpleMamba<B: Backend> {
  config: MambaConfig,
  embedding: Embedding<B>,
  projection: Linear<B>,
}

impl<B: Backend> SimpleMamba<B> {
  pub fn new(device: &B::Device, config: MambaConfig) -> Self {
    let embedding_config = EmbeddingConfig::new(config.vocab_size, config.d_model);
    let projection_config =
      LinearConfig::new(config.d_model, config.vocab_size).with_bias(config.bias);

    Self {
      config: config.clone(),
      embedding: embedding_config.init(device),
      projection: projection_config.init(device),
    }
  }

  pub fn forward(&self, input: Tensor<B, 2, Int>) -> Tensor<B, 3> {
    let [_batch_size, _seq_len] = input.dims();

    // 1. Input embedding
    let embedded = self.embedding.forward(input);

    // 2. For now, no specific Mamba processing

    // 3. Projection to vocabulary space
    let logits = self.projection.forward(embedded);

    logits
  }
}
