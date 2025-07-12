// brain.rs
use std::process::{Command, Stdio};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn think(input: &str, _config: &str, _memory: &crate::memory::Memory) -> String {
    let model_path = "brain/llama-3-8b-instruct.Q4_K_M.gguf";

    let mut cmd = Command::new("./llama.cpp/main") // Adjust path if needed
        .args(&[
            "-m", model_path,
            "-p", input,
            "-n", "128",
            "--temp", "0.7"
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run LLM");

    let stdout = cmd.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    let mut output = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("### Response:") {
            continue;
        }
        output.push_str(&line);
        output.push('\n');
    }

    output.trim().to_string()
}
