use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Future arguments can be added here
}

fn main() {
    let args = Args::parse();
    println!("Seeding the database with args: {:?}", args);
}
