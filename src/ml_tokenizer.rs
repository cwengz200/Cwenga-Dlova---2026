use anyhow::Result;
use std::collections::HashMap;

use tokenizers::models::wordlevel::WordLevelBuilder;
use tokenizers::normalizers::unicode::NFC;
use tokenizers::pre_tokenizers::whitespace::Whitespace;
use tokenizers::Tokenizer;

pub fn build_tokenizer(corpus: &[String], vocab_size: usize) -> Result<Tokenizer> {
    // 1) Count token frequencies (simple whitespace tokens)
    let mut freq: HashMap<String, usize> = HashMap::new();

    for line in corpus {
        let normalized = line.to_lowercase();
        for tok in normalized.split_whitespace() {
            *freq.entry(tok.to_string()).or_insert(0) += 1;
        }
    }

    // 2) Sort by frequency (descending)
    let mut items: Vec<(String, usize)> = freq.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));

    // 3) Build vocab map: token -> id
    let mut vocab: HashMap<String, u32> = HashMap::new();
    vocab.insert("[UNK]".to_string(), 0);
    vocab.insert("[PAD]".to_string(), 1);

    // 4) Add most frequent tokens up to vocab_size
    let mut next_id: u32 = 2;
    let limit = vocab_size.saturating_sub(2);

    for (tok, _) in items.into_iter().take(limit) {
        if !vocab.contains_key(&tok) {
            vocab.insert(tok, next_id);
            next_id += 1;
        }
    }

    // 5) Create WordLevel model via builder (tokenizers 0.15 compatible)
    let model = WordLevelBuilder::default()
        .vocab(vocab)
        .unk_token("[UNK]".to_string())
        .build().map_err(anyhow::Error::msg)?;

    let mut tokenizer = Tokenizer::new(model);

    // 6) Optional: normalizer + pre-tokenizer
    tokenizer.with_normalizer(NFC);
    tokenizer.with_pre_tokenizer(Whitespace);

    Ok(tokenizer)
}