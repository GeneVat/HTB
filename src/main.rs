use std::env;
use std::process::{Command, ExitCode};

/// Runs a specific cargo binary and passes the file_path argument to it.
fn run(bin: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--bin", bin, "--", file_path])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo run --bin {} failed", bin).into())
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Please provide a file path.");
        eprintln!("Usage: HTB <filename>");
        return ExitCode::from(1);
    }
    
    let file_path = &args[1];

    // Chain the execution of Step0 through Step4 sequentially, passing the file_path
    let result = run("Step0", file_path)
        .and_then(|_| run("Step1", file_path))
        .and_then(|_| run("Step2", file_path))
        .and_then(|_| run("Step3", file_path))
        .and_then(|_| run("Step4", file_path));

    if let Err(e) = result {
        eprintln!("Execution failed: {e}");
        return ExitCode::from(1);
    }

    ExitCode::from(0)
}