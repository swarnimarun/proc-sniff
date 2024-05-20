use clap::{Parser, Subcommand};
use proc_sniff;
use sysinfo::System;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct App {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Links {
        #[clap(subcommand)]
        i: ProcessIdentifier,
    },
    Ports {
        #[clap(subcommand)]
        i: ProcessIdentifier,
    },
}

#[derive(Clone, Subcommand, Debug)]
enum ProcessIdentifier {
    Name { d: String },
    Pid { d: i32 },
}

fn main() {
    let p = App::parse();
    let mut sys = System::new();
    match p.commands {
        Commands::List => {
            for (pid, name) in proc_sniff::procs::get_procs(&mut sys) {
                println!("{:>5}: {name}", pid.to_string());
            }
        }
        Commands::Links {
            i: ProcessIdentifier::Pid { d },
        } => {
            if let Ok(set) = proc_sniff::links::get_links(d) {
                for link in set {
                    println!("> {link}");
                }
            }
        }
        Commands::Links {
            i: ProcessIdentifier::Name { d },
        } => todo!(),
        Commands::Ports {
            i: ProcessIdentifier::Pid { d },
        } => {
            for port in proc_sniff::ports::get_ports(d as u32) {
                println!("> {port}");
            }
        }
        Commands::Ports {
            i: ProcessIdentifier::Name { d },
        } => todo!(),
    }
}
