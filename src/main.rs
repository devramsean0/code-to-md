use std::{fs, path::Path};

use clap::Parser;

mod walker;

#[derive(Parser)]
struct Args {
    #[arg()]
    path: String,
}

fn main() {
    let args = Args::parse();
    let mut items: Vec<Item> = vec![];

    let mut walk = walker::Walk::new(Path::new(&args.path));
    while let Some(p) = walk.next() {
        if !p.is_dir() {
            if let Ok(file) = fs::read_to_string(p.clone().as_path()) {
                items.push(Item {
                    path: p.to_string_lossy().to_string(),
                    contents: file,
                });
            }
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
