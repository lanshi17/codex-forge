//! Synthetic JSONL producer for the live process supervisor tests.
use clap::Parser;
use std::{
    io::{self, Read, Write},
    process::ExitCode,
    thread,
    time::Duration,
};
#[derive(Parser)]
struct Args {
    #[arg(long)]
    scenario: String,
}
fn main() -> ExitCode {
    let args = Args::parse();
    if args.scenario == "stdin-blocked" {
        thread::sleep(Duration::from_secs(2));
        return ExitCode::SUCCESS;
    }
    if args.scenario == "heartbeat" {
        for _ in 0..200 {
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("heartbeat")
                .unwrap();
            f.write_all(b"x").unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        return ExitCode::SUCCESS;
    }
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    match args.scenario.as_str() {
        "incomplete" => {
            println!("{{\"type\":\"session\",\"sessionId\":\"incomplete\"}}");
            ExitCode::SUCCESS
        }
        "failure" => {
            print!(
                "{}",
                include_str!("../../tests/fixtures/dsh-live-smoke.jsonl")
            );
            ExitCode::from(7)
        }
        "timeout" => loop {
            thread::sleep(Duration::from_millis(10));
        },
        "malformed" => {
            println!("not-json");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(2));
            ExitCode::SUCCESS
        }
        "env" => {
            if std::env::var_os("HOME").is_some() {
                return ExitCode::from(3);
            }
            print!(
                "{}",
                include_str!("../../tests/fixtures/dsh-live-smoke.jsonl")
            );
            io::stdout().flush().unwrap();
            ExitCode::SUCCESS
        }
        "flood" => {
            let mut e = io::stderr();
            for _ in 0..100 {
                e.write_all(&[b'x'; 4096]).unwrap();
            }
            e.flush().unwrap();
            ExitCode::SUCCESS
        }
        "descendant" => {
            print!(
                "{}",
                include_str!("../../tests/fixtures/dsh-live-smoke.jsonl")
            );
            io::stdout().flush().unwrap();
            // Intentional orphan: this fixture tests supervisor cleanup after the parent exits.
            #[allow(clippy::zombie_processes)]
            let _child = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--scenario", "heartbeat"])
                .stdin(std::process::Stdio::null())
                .spawn()
                .unwrap();
            for _ in 0..100 {
                if std::path::Path::new("heartbeat").exists() {
                    break;
                }
                thread::sleep(Duration::from_millis(2));
            }
            ExitCode::SUCCESS
        }
        "success" => {
            print!(
                "{}",
                include_str!("../../tests/fixtures/dsh-live-smoke.jsonl")
            );
            io::stdout().flush().unwrap();
            ExitCode::SUCCESS
        }
        _ => ExitCode::from(2),
    }
}
