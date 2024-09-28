use clap::{App, Arg};
use std::process::Command;

fn main() {
    let matches = App::new("git-clean")
        .version("1.0")
        .author("You <you@example.com>")
        .about("Does awesome things with git")
        .arg(
            Arg::with_name("BRANCH")
                .help("Sets the branch name to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Obtiene el valor del argumento BRANCH
    let branch = matches.value_of("BRANCH").unwrap();

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

    println!("{}", String::from_utf8_lossy(&output.stdout))
}
