use std::process::{Command, ExitCode};

fn run(bin: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--bin", bin])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo run --bin {} failed", bin).into())
    }
}

fn main() -> ExitCode {
    if let Err(e) = run("Step0").and_then(|_| run("Step1")).and_then(|_| run("Step2")).and_then(|_| run("Step3")).and_then(|_| run("Step4")) {
        eprintln!("{e}");
        return ExitCode::from(1);
    }
    ExitCode::from(0)
}
