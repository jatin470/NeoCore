// core.rs
mod engine;
mod memory;
mod brain;

use std::fs;
use engine::execute_task;
use brain::think;
use memory::Memory;

fn main() {
    let config = fs::read_to_string("config.toml").expect("Config not found");
    let memory = Memory::load("memory/data.json");

    println!("🤖 NeoCore booted.\n");

    loop {
        println!("\n🗣️  Enter your command:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("👋 Bye.");
            break;
        }

        let response = think(&input, &config, &memory);
        println!("🧠 Neo: {}", response);

        execute_task(&response);
    }
}
