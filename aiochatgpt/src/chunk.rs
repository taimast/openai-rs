use serde::Deserialize;

const CHUNK_START: usize = 6;
const DONE_MARKER: &[u8; 1] = b"]";
const CHUNK_DELIMITER: u8 = b'\n';
const CHUNK_PREFIX: &[u8; 1] = b"\n";

type ResultOrError<T> = Result<T, Box<dyn std::error::Error>>;

// consume chunks. async consumer

#[derive(Debug, Deserialize, Default)]
pub struct ChatCompletionChunk {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<CompletionChoice>,
}


#[derive(Debug, Deserialize)]
struct CompletionChoice {
    delta: Delta,
    index: i64,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Delta {
    content: Option<String>,
}

/// A stream of chat completion chunks
impl ChatCompletionChunk {
    /// Returns the content of the first choice
    pub fn get_content(&self) -> Option<String> {
        self.choices[0].delta.content.clone()
    }


    /// Parse a chunk of bytes into a ChatCompletionChunk.
    ///
    /// todo L1 TODO 03.04.2023 22:38 taima: "Need remove first_chunk param and use some other way to detect first chunk"
    ///
    pub fn from_chunk(raw_chunk: &[u8]) -> ResultOrError<Vec<Self>> {
        let mut chunks = Vec::new();
        for mut chunk in raw_chunk.split(|&b| b == CHUNK_DELIMITER) {
            if chunk.starts_with(CHUNK_PREFIX) {
                chunk = &chunk[1..];
            }
            if chunk.is_empty() {
                continue;
            }
            let mut chunk = &chunk[CHUNK_START..];
            if chunk.ends_with(DONE_MARKER) {
                break;
            };
            let chunk = serde_json::from_slice(chunk)?;
            chunks.push(chunk);
        }
        Ok(chunks)
    }
}


