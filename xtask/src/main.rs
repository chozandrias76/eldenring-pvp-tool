use std::ffi::OsStr;
use std::sync::OnceLock;
use std::{env, fs, iter};

use anyhow::{bail, Context, Result};
use practice_tool_tasks::{
    cargo_command, project_root, steam_command, target_path, Distribution, FileInstall,
};
use toml::Value;

mod codegen;

const APPID: u32 = 1245620;
pub static RUNTIME_CONFIG_FILENAME: OnceLock<String> = OnceLock::new();

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config_content = include_str!("../../Cargo.toml");
    let config: Value = config_content.parse::<Value>().expect("Failed to parse Cargo.toml");
    let invasion_tool_config = config.get("er_invasion_tool").expect("er_invasion_tool section expected in Cargo.toml.");
    let runtime_config_filename = invasion_tool_config.get("config_file_name").and_then(Value::as_str).unwrap_or("er_invasion_tool.toml");
    RUNTIME_CONFIG_FILENAME.set(runtime_config_filename.to_string()).expect("Could not write runtime config filename");

    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist()?,
        Some("codegen") => codegen::codegen()?,
        Some("inject") => inject(env::args().skip(1).map(String::from))?,
        Some("run") => run()?,
        Some("install") => install()?,
        Some("uninstall") => uninstall()?,
        Some("help") => print_help(),
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"
Tasks:

run ............. compile and start the practice tool
dist ............ build distribution artifacts
codegen ......... generate Rust code: parameters, base addresses, ...
inject <args> ... standalone dll inject
install ......... install standalone dll to $ER_PATH
uninstall ....... uninstall standalone dll from $ER_PATH
help ............ print this help
"#
    );
}

fn run() -> Result<()> {
    let runtime_config_filename = RUNTIME_CONFIG_FILENAME.get().expect("Could not read runtime config filename");
    let status = cargo_command("build")
        .args(["--lib", "--package", "eldenring-practice-tool"])
        .status()
        .context("cargo")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    fs::copy(
        project_root().join(runtime_config_filename),
        target_path("debug").join(runtime_config_filename),
    )?;

    let dll_path = target_path("debug").join("libjdsd_er_practice_tool.dll").canonicalize()?;

    inject(iter::once(dll_path))?;

    Ok(())
}

fn dist() -> Result<()> {
    let runtime_config_filename = RUNTIME_CONFIG_FILENAME.get().expect("Could not read runtime config filename");
    Distribution::new("jdsd_er_practice_tool.zip")
        .with_artifact("libjdsd_er_practice_tool.dll", "jdsd_er_practice_tool.dll")
        .with_artifact("jdsd_er_practice_tool.exe", "jdsd_er_practice_tool.exe")
        .with_file("lib/data/RELEASE-README.txt", "README.txt")
        .with_file(runtime_config_filename, runtime_config_filename)
        .build(&["--locked", "--release", "--workspace", "--exclude", "xtask"])
}

fn install() -> Result<()> {
    let runtime_config_filename = RUNTIME_CONFIG_FILENAME.get().expect("Could not read runtime config filename");
    let status = cargo_command("build")
        .args(["--lib", "--release", "--package", "eldenring-practice-tool"])
        .status()
        .context("cargo")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    FileInstall::new()
        .with_file(target_path("release").join("libjdsd_er_practice_tool.dll"), "dinput8.dll")
        .with_file(project_root().join(runtime_config_filename), runtime_config_filename)
        .install("ER_PATH")?;

    Ok(())
}

fn uninstall() -> Result<()> {
    let runtime_config_filename = RUNTIME_CONFIG_FILENAME.get().expect("Could not read runtime config filename");
    FileInstall::new()
        .with_file(target_path("release").join("libjdsd_er_practice_tool.dll"), "dinput8.dll")
        .with_file(project_root().join(runtime_config_filename), runtime_config_filename)
        .uninstall("ER_PATH")?;

    Ok(())
}

fn inject<S: AsRef<OsStr>>(args: impl Iterator<Item = S>) -> Result<()> {
    cargo_command("build").args(["--release", "--bin", "inject"]).status().context("cargo")?;

    steam_command(target_path("release").join("inject"), APPID)?
        .args(args)
        .status()
        .context("inject")?;

    Ok(())
}
