use anyhow::Result;

use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor, TensorData};

use crate::events::Event;
use crate::ml_dataset::build_month_samples;
use crate::ml_model::{MonthClassifier, MonthClassifierConfig};
use crate::ml_tokenizer::build_tokenizer;

// ------------------------------------------------------------
// Encode texts -> Tensor<[batch, max_len], Int>
// ------------------------------------------------------------
fn encode_texts<B: Backend>(
    tokenizer: &tokenizers::Tokenizer,
    texts: &[String],
    max_len: usize,
    device: &B::Device,
) -> Tensor<B, 2, Int> {
    // We'll build a flat Vec<usize> of length batch * max_len
    let mut all_ids: Vec<usize> = Vec::with_capacity(texts.len() * max_len);

    for t in texts {
        let enc = tokenizer.encode(t, false).unwrap();

        // token ids from tokenizers are u32; convert to usize for TensorData
        let mut ids: Vec<i64> = enc.get_ids().iter().map(|&x: u32| x as i64).collect();
        // pad / truncate
        ids.truncate(max_len);
        while ids.len() < max_len {
            ids.push(1usize); // PAD = 1
        }

        all_ids.extend(ids);
    }

    // Burn 0.20.1: use TensorData::new(data, shape)
    let data = TensorData::new(all_ids, [texts.len(), max_len]);

    Tensor::<B, 2, Int>::from_data(data, device)
}

// ------------------------------------------------------------
// Step 1/2-ish: Build dataset + tokenizer + tensors + model init
// (We keep it simple and compiling; training loop can be added later.)
// ------------------------------------------------------------
pub fn train_month_classifier<B: Backend>(
    events: &[Event],
    epochs: usize,
    lr: f64,
    device: &B::Device,
) -> Result<()> {
    // Build dataset
    let samples = build_month_samples(events);
    let (texts, labels): (Vec<String>, Vec<usize>) = samples.into_iter().unzip();

    // Tokenizer
    let vocab_size: usize = 5000;
    let tokenizer = build_tokenizer(&texts, vocab_size)?;

    // Encode inputs
    let max_len: usize = 32;
    let x: Tensor<B, 2, Int> = encode_texts::<B>(&tokenizer, &texts, max_len, device);

    // Labels tensor (shape [batch])
    // IMPORTANT: use Vec<usize> + TensorData::new (NOT TensorData::from(Vec<i64>))
    // Burn TensorData expects elements like i64/u32/f32 etc. (not usize)
    let labels_i64: Vec<i64> = labels.iter().map(|&v| v as i64).collect();
    let y_data = TensorData::new(labels_i64, [labels.len()]);
    let y: Tensor<B, 1, Int> = Tensor::<B, 1, Int>::from_data(y_data, device);

    // Model config (NO max_len field here)
    let cfg = MonthClassifierConfig {
        vocab_size,
        d_model: 128,
        n_heads: 4,
        n_layers: 6,
        d_ff: 256,
        dropout: 0.1,
        pad_id: 1,
    };

    let _model: MonthClassifier<B> = cfg.init(device);

    // For now: just confirm tensors look correct
    println!(
        "Prepared training pipeline: samples={}, epochs={}, lr={}, x_shape=[{}, {}], y_len={}",
        texts.len(),
        epochs,
        lr,
        x.dims()[0],
        x.dims()[1],
        y.dims()[0],
    );

    Ok(())
}