# Word DOCX Question Answering System
## Transformer-Based Month Classification using Burn (Rust)

### 👩🏾‍💻 Author
Cwenga Dlova  
PGD ICT – Software Engineering  
Cape Peninsula University of Technology  

---

## 📌 Project Overview

This project implements a Rust-based Question & Answer system over Microsoft DOCX calendar documents, extended with a Transformer-based month classification model built using the Burn deep learning framework (v0.20.1).

The system combines:

- Document parsing
- Structured data extraction
- Rule-based question answering
- Neural network architecture design
- CLI interface for interaction

---

## 🎯 Objectives

1. Load and extract structured data from DOCX calendar files.
2. Answer natural language queries over event data.
3. Design a Transformer encoder model for month classification.
4. Implement a modular, scalable Rust architecture.
5. Demonstrate ML pipeline integration in a systems programming environment.

---

## 🏗️ System Architecture
src/
│
├── main.rs → CLI orchestration
├── doc_loader.rs → DOCX extraction
├── events.rs → Event struct + parsing
├── qa.rs → Rule-based Q&A engine
├── ml_dataset.rs → Training data generation
├── ml_tokenizer.rs → Tokenizer builder
├── ml_model.rs → Transformer classifier
└── ml_train.rs → Training pipeline


The architecture follows separation of concerns and modular design principles.

---

## 🤖 Machine Learning Model

### Model Type:
Transformer Encoder

### Configuration:
- Vocabulary Size: 5000
- Sequence Length: 32
- Embedding Dimension: 128
- Number of Heads: 4
- Number of Layers: 6
- Feed-forward Dimension: 256
- Dropout: 0.1
- Output Classes: 12 (Months)

### Pipeline:

Input Text → Tokenizer → Token IDs → Embedding → Transformer Encoder → Mean Pooling → Linear Layer → Month Prediction

---

## 📂 Data Pipeline

1. DOCX files extracted via ZIP processing.
2. XML content parsed into structured `Event` objects.
3. Event descriptions converted into token sequences.
4. Labels generated as month indices (0–11).
5. Tensor conversion using Burn's `TensorData`.

---

## 💻 How to Build

```bash
cargo build


The architecture follows separation of concerns and modular design principles.

---

## 🤖 Machine Learning Model

### Model Type:
Transformer Encoder

### Configuration:
- Vocabulary Size: 5000
- Sequence Length: 32
- Embedding Dimension: 128
- Number of Heads: 4
- Number of Layers: 6
- Feed-forward Dimension: 256
- Dropout: 0.1
- Output Classes: 12 (Months)

### Pipeline:

Input Text → Tokenizer → Token IDs → Embedding → Transformer Encoder → Mean Pooling → Linear Layer → Month Prediction

---

## 📂 Data Pipeline

1. DOCX files extracted via ZIP processing.
2. XML content parsed into structured `Event` objects.
3. Event descriptions converted into token sequences.
4. Labels generated as month indices (0–11).
5. Tensor conversion using Burn's `TensorData`.

---

## 💻 How to Build

```bash
cargo build

▶️ How to Run
Load DOCX Files
cargo run -- load --docs <path-to-docs-folder>

Ask a Question
cargo run -- ask --docs <path-to-docs-folder> --question "When is New Year's Day?"

Train Month Classifier
cargo run -- train-month <docs-folder> <epochs> <learning-rate>

Predict Month
cargo run -- predict-month "Event description text"
