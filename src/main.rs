use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    branch: String,
}

fn main() {
    let args = Cli::parse();

    println!("{}", args.branch);

    let output = Command::new("git")
        .args(&["branch", "-D", "temp"])
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("Exit status: {}", output.status);

    let output = Command::new("git")
        .args(&["checkout", "-b", "temp"])
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("Exit status: {}", output.status);

    let output = Command::new("git")
        .args(&["branch", "-D", &args.branch])
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("Exit status: {}", output.status);

    let output = Command::new("git")
        .args(&["fetch", "origin", &args.branch])
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("Exit status: {}", output.status);

    let output = Command::new("git")
        .args(&["checkout", &args.branch])
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("Exit status: {}", output.status);
}
