use clap::Parser;

#[derive(Parser)]
struct Args {}

fn main() {
    let args = Args::parse();
}

enum ItemType {
    File,
    Folder,
}

struct ItemType {
    i_type: String,
}
