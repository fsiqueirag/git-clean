use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    branch: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.branch);
    Command::new("git")
        .args(&["branch", "-D", "temp"])
        .output()
        .expect("failed to execute process");

    Command::new("git")
        .args(&["checkout", "-b", "temp"])
        .output()
        .expect("failed to execute process");

    Command::new("git")
        .args(&["branch", "-D", branch])
        .output()
        .expect("failed to execute process");

    Command::new("git")
        .args(&["fetch", "origin", branch])
        .output()
        .expect("failed to execute process");

    let output = Command::new("git")
        .args(&["checkout", branch])
        .output()
        .expect("failed to execute process");

    // println!("{}", String::from_utf8_lossy(&output.stdout))
}
