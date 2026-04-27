use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// A string argument
    #[arg(short, long)]
    string_arg: String,

    /// A boolean argument
    #[arg(short)]
    bool_arg: bool,

    /// An optional integer argument
    #[arg(short)]
    opt_arg: Option<i32>,

    /// A list of integers
    #[arg(short, long = "list")]
    list_arg: Vec<i32>,
}

fn main() {
    // INFO: cargo run --example clap_ -- --string-arg "hello" --list 1 --list 2 -o 3 -b
    // NOTE: if -b is not provided, bool_arg will be false
    // NOTE: --string-arg is equivalent to -s
    let args = Args::parse();
    println!("{:#?}", args);
}
