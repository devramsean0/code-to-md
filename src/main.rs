use std::fs;

use clap::Parser;
use log::error;

#[derive(Parser)]
struct Args {
    #[arg()]
    path: String,

    #[arg(short, long)]
    desc_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut items: Vec<Item> = vec![];

    for result in ignore::WalkBuilder::new(args.path)
        .sort_by_file_path(|a, b| {
            // Directories first, then files alphabetically
            match (a.is_dir(), b.is_dir()) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        })
        .build()
    {
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
    if args.desc_file.is_some() {
        let header = fs::read_to_string(args.desc_file.unwrap()).unwrap();
        md_contents.push(header);
    }
    for item in items {
        md_contents.push(format!("## {}", item.path));
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
