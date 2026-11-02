use btae_eval::{Budget, ClusterPair, Manifest, Observation};
use clap::{Parser, Subcommand};
use serde::{Deserialize, de::DeserializeOwned};
use std::{collections::BTreeSet, error::Error, path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[command(
    version,
    about = "Offline BTAE experiment preparation; no model execution"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run an isolated synthetic worker, not a real study or model.
    #[cfg(target_os = "linux")]
    Diagnose(Box<DiagnosticArgs>),
    /// Check a diagnostic's file hashes and its binding to a frozen run plan.
    #[cfg(target_os = "linux")]
    VerifyDiagnostic {
        directory: PathBuf,
        manifest: PathBuf,
    },
    /// Validate draft inputs, allowing unresolved launch blockers.
    Validate { manifest: PathBuf },
    /// Check launch prerequisites. Draft/unreviewed studies return nonzero.
    CheckReady { manifest: PathBuf },
    /// Emit a deterministic randomized run plan; never starts workers.
    Plan { manifest: PathBuf },
    /// Score evaluator-collected input. Worker reports are not accepted as evidence.
    Score { input: PathBuf },
    /// Exploratory repository-level paired bootstrap on external aggregate inputs.
    Bootstrap {
        input: PathBuf,
        #[arg(long, default_value_t = 20261102)]
        seed: u64,
        #[arg(long, default_value_t = 10000)]
        resamples: usize,
    },
}

#[cfg(target_os = "linux")]
#[derive(clap::Args)]
struct DiagnosticArgs {
    #[arg(long, default_value = "datasets/pilot.json")]
    manifest: PathBuf,
    #[arg(long)]
    fixture_worker: PathBuf,
    #[arg(long)]
    output: PathBuf,
    #[arg(long, default_value = "queue-r01")]
    task: String,
    #[arg(long, default_value = "A")]
    arm: String,
    #[arg(long, default_value_t = 1)]
    repeat: u32,
    #[arg(long, value_enum, default_value = "success")]
    scenario: btae_eval::runner::Scenario,
    #[arg(long, default_value_t = 5000)]
    wall_ms: u64,
    #[arg(long, default_value_t = 32)]
    tokens: u64,
    #[arg(long)]
    cancel_file: Option<PathBuf>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScoreInput {
    expected_impact: BTreeSet<String>,
    budget: Budget,
    observation: Observation,
}

fn read<T: DeserializeOwned>(p: PathBuf) -> Result<T, Box<dyn Error + Send + Sync>> {
    Ok(serde_json::from_slice(&std::fs::read(p)?)?)
}
fn print(value: &impl serde::Serialize) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
fn run(cli: Cli) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cli.command {
        #[cfg(target_os = "linux")]
        Command::Diagnose(args) => {
            let request = btae_eval::runner::DiagnosticRequest {
                manifest: args.manifest,
                task_id: args.task,
                arm: args.arm,
                repeat: args.repeat,
                fixture_worker: args.fixture_worker,
                output: args.output,
                scenario: args.scenario,
                limits: btae_eval::runner::Limits {
                    wall_ms: args.wall_ms,
                    tokens: args.tokens,
                    ..Default::default()
                },
                host_probe: None,
                host_address: None,
                cancel_file: args.cancel_file,
            };
            print(&btae_eval::runner::run_diagnostic(request)?)?;
        }
        #[cfg(target_os = "linux")]
        Command::VerifyDiagnostic {
            directory,
            manifest,
        } => {
            let record = btae_eval::runner::verify_diagnostic(&directory, &manifest)?;
            print(
                &serde_json::json!({"valid_diagnostic":true,"run_id":record.run.run_id,
                "data_kind":record.data_kind,"outcome":record.outcome,"empirical_task_success":null}),
            )?;
        }
        Command::Validate { manifest } => {
            let m: Manifest = read(manifest)?;
            m.validate()?;
            print(
                &serde_json::json!({"input_valid":true,"declarations_complete":m.blockers().is_empty(),"blockers":m.blockers(),"note":"Validation does not launch a study or verify host enforcement."}),
            )?;
        }
        Command::CheckReady { manifest } => {
            let m: Manifest = read(manifest)?;
            let blockers = m.blockers();
            print(
                &serde_json::json!({"declarations_complete":blockers.is_empty(),"blockers":blockers,
                "note":"This checks declarations only; host-enforced isolation and evidence must be audited separately."}),
            )?;
            if !blockers.is_empty() {
                return Err("study is not ready for empirical execution".into());
            }
        }
        Command::Plan { manifest } => {
            let bytes = std::fs::read(manifest)?;
            let m = serde_json::from_slice(&bytes)?;
            print(&btae_eval::plan(&m, &bytes)?)?;
        }
        Command::Score { input } => {
            let input: ScoreInput = read(input)?;
            print(&btae_eval::score(
                &input.observation,
                &input.expected_impact,
                &input.budget,
            )?)?;
        }
        Command::Bootstrap {
            input,
            seed,
            resamples,
        } => {
            let pairs: Vec<ClusterPair> = read(input)?;
            print(&btae_eval::cluster_bootstrap(&pairs, seed, resamples)?)?;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("btae-eval: {e}");
            ExitCode::FAILURE
        }
    }
}
