use burn::tensor::{Tensor, backend::Backend};

/// Creates a causal mask to ensure the model doesn't look into the future
pub fn create_causal_mask<B: Backend>(seq_len: usize, device: &B::Device) -> Tensor<B, 2> {
  // Create a square matrix filled with ones
  let ones = Tensor::<B, 2>::ones([seq_len, seq_len], device);
  // Extract the upper triangular part (excluding diagonal) which represents future tokens
  let triu = Tensor::<B, 2>::triu(ones, 1);
  // Multiply by a large negative value to create the mask
  // This ensures future tokens get -infinity in attention weights
  // which results in zero probability after softmax
  let mask = triu.mul_scalar(-1e9);
  mask
}

/// Applies the mask to the logits before the softmax calculation
pub fn apply_mask<B: Backend>(logits: Tensor<B, 3>, mask: Tensor<B, 2>) -> Tensor<B, 3> {
  // Extend mask to match logits dimensions
  // We need to unsqueeze the mask from shape [seq_len, seq_len] to [seq_len, seq_len, 1]
  // This allows broadcasting when we add it to the logits tensor of shape [batch, seq_len, vocab_size]
  // Without this dimension adjustment, we couldn't add the mask to the logits tensor
  let mask = mask.unsqueeze::<3>();
  logits + mask
}
