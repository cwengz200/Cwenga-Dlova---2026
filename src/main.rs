
use anyhow::Result;
use clap::{Parser, Subcommand};

mod doc_loader;
mod events;
mod qa;
mod ml_dataset;
mod ml_tokenizer;
mod ml_model;
mod ml_train;

#[derive(Parser, Debug)]
#[command(name = "word-doc-qa")]
#[command(about = "Rust Q&A over DOCX calendar documents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Load DOCX files and print extracted text
    Load {
        #[arg(long)]
        docs: String,
    },
    /// Ask a question over the DOCX calendars
    Ask {
        #[arg(long)]
        docs: String,
        #[arg(long)]
        question: String,
    },
    TrainMonth {
        #[arg(long)]
        docs: String,
        #[arg(long)]
        epochs: usize,
        #[arg(long)]
        lr: f64,
    },
    PredictMonth {
        #[arg(long)]
        text: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Load { docs } => {
            let docs = doc_loader::load_docx_folder(&docs)?;
            println!("Loaded {} documents\n", docs.len());
            for (i, d) in docs.iter().enumerate() {
                println!("--- DOC {}: {} ---", i + 1, d.name);
                for (line_idx, line) in d.text.lines().take(40).enumerate() {
                    println!("{:02}: {}", line_idx + 1, line);
                }
                println!("(…truncated…)\n");
            }
        }

        Commands::Ask { docs, question } => {
            let docs = doc_loader::load_docx_folder(&docs)?;
            let mut all_events = Vec::new();

            for d in &docs {
                let mut events = events::parse_events_from_text(&d.name, &d.text);
                all_events.append(&mut events);
            }

            println!("Parsed {} events.", all_events.len());
            let answer = qa::answer_question(&question, &all_events);
            println!("\nQ: {}\nA:\n{}", question, answer);
        }
        Commands::TrainMonth { docs, epochs, lr } => {
            println!("Training month model with docs={}, epochs={}, lr={}", docs, epochs, lr);
        }
        Commands::PredictMonth { text } => {
            println!("Predicting month for text: {}", text);
        }
    }

    Ok(())
}