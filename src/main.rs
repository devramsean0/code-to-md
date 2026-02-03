use std::fs;

use clap::Parser;
use log::error;

#[derive(Parser)]
struct Args {
    #[arg()]
    path: String,
}

fn main() {
    let args = Args::parse();
    let mut items: Vec<Item> = vec![];

    for result in ignore::Walk::new(args.path) {
        match result {
            Ok(p) => {
                if !p.path().is_dir() {
                    if let Ok(file) = fs::read_to_string(p.clone().path()) {
                        items.push(Item {
                            path: p.path().to_string_lossy().to_string(),
                            contents: file,
                        });
                    }
                }
            }
            Err(e) => error!("Error walking tree: {e}"),
        }
    }
    println!("Found {} files", items.len());

    // Incredibly awful templater
    let mut md_contents: Vec<String> = vec![];
    for item in items {
        md_contents.push(format!("# {}", item.path));
        let extension = item.path.split(".").last().unwrap();
        md_contents.push(format!("```{extension}\n{}\n```", item.contents))
    }

    fs::write("output.md", md_contents.join("\n")).unwrap();
}

#[derive(Debug)]
struct Item {
    path: String,
    contents: String,
}
