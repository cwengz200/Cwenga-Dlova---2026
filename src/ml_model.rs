use burn::module::Module;
use burn::nn;
use burn::tensor::{backend::Backend, Tensor};

use burn::nn::transformer::{
    TransformerEncoder, TransformerEncoderConfig, TransformerEncoderInput,
};

#[derive(Module, Debug)]
pub struct MonthClassifier<B: Backend> {
    embedding: nn::Embedding<B>,
    encoder: TransformerEncoder<B>,
    head: nn::Linear<B>,
    pad_id: i64,
}

#[derive(Debug, Clone)]
pub struct MonthClassifierConfig {
    pub vocab_size: usize,
    pub d_model: usize,
    pub n_heads: usize,
    pub n_layers: usize, // MUST be 6
    pub d_ff: usize,
    pub dropout: f64,
    pub pad_id: i64, // [PAD] id (we use 1)
}

impl MonthClassifierConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> MonthClassifier<B> {
        // Encoder config in burn 0.20.1 expects 4 args:
        // (vocab_size?, d_model, n_heads, d_ff)
        // Based on your earlier error, burn wants exactly 4 parameters here.
        let mut enc_cfg = TransformerEncoderConfig::new(
            self.vocab_size,
            self.d_model,
            self.n_heads,
            self.d_ff,
        );

        // In your build, these are config fields (no .with_n_layers() method)
        enc_cfg.n_layers = self.n_layers;
        enc_cfg.dropout = self.dropout;

        MonthClassifier {
            embedding: nn::EmbeddingConfig::new(self.vocab_size, self.d_model).init(device),
            encoder: enc_cfg.init(device),
            head: nn::LinearConfig::new(self.d_model, 12).init(device),
            pad_id: self.pad_id,
        }
    }
}

impl<B: Backend> MonthClassifier<B> {
    /// input_ids: [batch, seq] int (token ids)
    pub fn forward(&self, input_ids: Tensor<B, 2, burn::tensor::Int>) -> Tensor<B, 2> {
        // Embed tokens -> [batch, seq, d_model]
        let x = self.embedding.forward(input_ids);

        // Burn 0.20.1 in your build expects only the tensor in TransformerEncoderInput::new
        let enc_in = TransformerEncoderInput::new(x);

        // [batch, seq, d_model]
        let hidden = self.encoder.forward(enc_in);

        // mean over seq -> [batch, 1, d_model] then squeeze -> [batch, d_model]
        let pooled = hidden.mean_dim(1).squeeze();

        self.head.forward(pooled)
    }
}