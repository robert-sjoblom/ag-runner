use structopt::StructOpt;
use std::process::Command;
use std::process;

#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let status = Command::new("ag")
                         .arg("@")
                         .arg(&args.path)
                         .arg("-H")
                         .status()
                         .unwrap();

    // we switch because ag will return exit 0 on finding something
    if status.success() {
        println!("{:#?} partial contains instance variables", &args.path);
        process::exit(1);
    } else {
        println!("Everything looks okay!")
    }
}
