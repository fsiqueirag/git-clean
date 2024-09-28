use clap::Parser;
use colored::*;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    branch: String,
}

fn main() {
    let args = Cli::parse();
    println!("{} {} !", "it".green(), "works".blue().bold());

    Command::new("git")
        .args(&["branch", "-D", "temp"])
        .output()
        .expect("failed to execute process");
    println!("Deleted branch temp");

    Command::new("git")
        .args(&["checkout", "-b", "temp"])
        .output()
        .expect("failed to execute process");
    println!("Created branch temp");

    Command::new("git")
        .args(&["branch", "-D", &args.branch])
        .output()
        .expect("failed to execute process");
    println!("Deleted branch {}", args.branch);

    Command::new("git")
        .args(&["fetch", "origin", &args.branch])
        .output()
        .expect("failed to execute process");
    println!("Fetched branch {}", args.branch);

    Command::new("git")
        .args(&["checkout", &args.branch])
        .output()
        .expect("failed to execute process");

    println!("Checked out at branch {}", args.branch);
    println!(
        "{} {}!",
        "Succesfully cleaned branch ".green(),
        args.branch.blue().bold()
    )
}
