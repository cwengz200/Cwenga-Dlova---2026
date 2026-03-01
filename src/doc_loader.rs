use anyhow::{anyhow, Context, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(Debug, Clone)]
pub struct LoadedDoc {
    pub name: String,
    pub text: String,
}

pub fn load_docx_folder(folder: &str) -> Result<Vec<LoadedDoc>> {
    let mut out = vec![];
    let dir = Path::new(folder);

    if !dir.exists() {
        return Err(anyhow!("Docs folder not found: {}", folder));
    }

    for entry in fs::read_dir(dir).context("Reading docs folder")? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("docx") {
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown.docx")
                .to_string();

            let text = extract_docx_text(&path)
                .with_context(|| format!("Extracting text from {}", name))?;

            out.push(LoadedDoc { name, text });
        }
    }

    if out.is_empty() {
        return Err(anyhow!("No .docx files found in {}", folder));
    }

    Ok(out)
}

fn extract_docx_text(path: &PathBuf) -> Result<String> {
    let file = fs::File::open(path)?;
    let mut zip = ZipArchive::new(file).context("DOCX is not a valid zip")?;

    let mut xml = String::new();
    zip.by_name("word/document.xml")
        .context("word/document.xml not found in docx")?
        .read_to_string(&mut xml)?;

    extract_text_from_document_xml(&xml)
}

fn extract_text_from_document_xml(xml: &str) -> Result<String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut out = String::new();
    let mut in_text = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if e.name().as_ref().ends_with(b":t") || e.name().as_ref() == b"w:t" {
                    in_text = true;
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref().ends_with(b":t") || e.name().as_ref() == b"w:t" {
                    in_text = false;
                }
                if e.name().as_ref().ends_with(b":p") || e.name().as_ref() == b"w:p" {
                    out.push('\n');
                }
            }
            Ok(Event::Text(t)) => {
                if in_text {
                    out.push_str(&t.unescape()?.to_string());
                    out.push(' ');
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow!("XML parse error: {e}")),
            _ => {}
        }
        buf.clear();
    }

    Ok(out)
}