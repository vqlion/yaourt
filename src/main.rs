/*
yaourt

SSH connection manager. Parses the SSH known host file, gives the list of hosts and connects to the one you choose.

Santé, bonheur, prospérité, ***********
*/

use clap::Parser;
use colored::Colorize;
use ssh_key::KnownHosts;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const SSH_RELATIVE_PATH: &str = ".ssh/known_hosts";

#[derive(Parser)]
#[command(
    about,
    version,
    after_help = "> yaourt parses your ssh known hosts file and lets you search and connect to any host in it"
)]
struct Cli {
    /// Search for a host
    #[arg(default_value = "")]
    search: String,

    /// Use a different username for ssh login
    #[arg(short, long, default_value = "")]
    login_name: String,
}

fn main() {
    let args = Cli::parse();

    let search: String = args.search;
    let ssh_user: String = args.login_name;
    let mut seen_addresses: Vec<String> = Vec::new();
    let mut displayed_addresses: Vec<String> = Vec::new();
    let mut iterator: i32 = 1;

    println!(
        "{}{}",
        "yaourt v".bright_magenta(),
        VERSION.bright_magenta().bold()
    );

    let ssh_file_vec =
        KnownHosts::read_file(get_ssh_file_path()).expect("Error reading SSH host file.");

    for entry in &ssh_file_vec {
        let host_patterns: &Vec<String>;
        match entry.host_patterns() {
            ssh_key::known_hosts::HostPatterns::Patterns(patt) => {
                host_patterns = patt;
            }
            ssh_key::known_hosts::HostPatterns::HashedName { salt: _, hash: _ } => panic!(
                "Error parsing SSH known hosts file: file is hashed. Set your SSH HashKnownHost option to 'no'"
            ),
        }
        let address = &host_patterns[0];
        if !seen_addresses.contains(&address) {
            seen_addresses.push(address.to_string());
            if search == "" || address.contains(&search) {
                println!(
                    "{}{} {}",
                    format!("{:>2}", iterator.to_string().bright_blue()),
                    ":".bright_blue(),
                    address
                );
                iterator += 1;
                displayed_addresses.push(address.to_string());
            }
        }
    }

    let int_choice: usize;

    if displayed_addresses.len() == 0 {
        println!("Couldn't find any host matching your request.");
        return;
    } else if displayed_addresses.len() == 1 {
        int_choice = 1;
    } else {
        println!("Choice?");
        let mut input_choice = String::new();
        io::stdin().read_line(&mut input_choice).unwrap();
        int_choice = input_choice.trim().parse::<usize>().unwrap();
    }
    let chosen_address = &displayed_addresses[int_choice - 1];

    println!(
        "Connecting to {}...\n",
        chosen_address.bright_green().bold()
    );

    let mut ssh_args: Vec<String> = Vec::new();
    if ssh_user != "" {
        ssh_args.push("-l".to_string());
        ssh_args.push(ssh_user);
    }
    ssh_args.push(chosen_address.to_string());

    if let Ok(mut child) = Command::new("ssh").args(ssh_args).spawn() {
        child.wait().expect("closed");
        println!("\n{}.", "Au revoir".bright_magenta().italic());
    } else {
        panic!("Couldn't start SSH session");
    }
}

fn get_ssh_file_path() -> PathBuf {
    match env::home_dir() {
        Some(path) => path.join(Path::new(SSH_RELATIVE_PATH)),
        None => panic!("Couldn't get the home dir."),
    }
}
