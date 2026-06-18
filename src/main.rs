// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::BTreeSet,
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::{self, Command},
};

const COMMON_COMMANDS: &[&str] = &[
    "ls",
    "cat",
    "grep",
    "find",
    "sort",
    "tar",
    "curl",
    "git",
    "bash",
    "wsl",
];

fn main() {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    let exit_code = match args.first().and_then(|arg| arg.to_str()) {
        None | Some("-h") | Some("--help") => {
            print_help();
            0
        }
        Some("-V") | Some("--version") => {
            println!("winux {}", env!("CARGO_PKG_VERSION"));
            0
        }
        Some("doctor") => doctor(),
        Some("which") => which_command(&args[1..]),
        Some("run") => run_command(&args[1..]),
        Some(command) => {
            eprintln!("unknown command: {command}");
            eprintln!();
            print_help();
            2
        }
    };

    process::exit(exit_code);
}

fn print_help() {
    println!(
        r#"winux - Linux-like CLI workflow manager for Windows

Usage:
  winux doctor              Inspect the current CLI environment
  winux which <command>     Show candidate executables found on PATH
  winux run <command> ...   Run a command and passthrough its exit code
  winux --help              Show this help
  winux --version           Show version

Examples:
  winux doctor
  winux which grep
  winux run grep -R todo .
"#
    );
}

fn doctor() -> i32 {
    println!("winux doctor");
    println!("============");
    println!("OS: {}", env::consts::OS);
    println!("ARCH: {}", env::consts::ARCH);

    match env::current_dir() {
        Ok(current_dir) => println!("Current directory: {}", current_dir.display()),
        Err(error) => println!("Current directory: <unavailable: {error}>"),
    }

    println!();
    println!("Shell hints:");
    print_env_hint("SHELL");
    print_env_hint("COMSPEC");
    print_env_hint("WT_SESSION");
    print_env_hint("TERM_PROGRAM");

    println!();
    println!("PATH summary:");
    let path_entries = env::var_os("PATH")
        .map(|value| env::split_paths(&value).collect::<Vec<_>>())
        .unwrap_or_default();
    println!("Entries: {}", path_entries.len());

    println!();
    println!("Common command candidates:");
    for command in COMMON_COMMANDS {
        let candidates = find_candidates(&OsString::from(command));
        if let Some(first) = candidates.first() {
            println!("  {command:<5} -> {}", first.display());
            if candidates.len() > 1 {
                println!("        + {} more candidate(s)", candidates.len() - 1);
            }
        } else {
            println!("  {command:<5} -> not found");
        }
    }

    0
}

fn print_env_hint(name: &str) {
    match env::var(name) {
        Ok(value) if !value.is_empty() => println!("  {name}: {value}"),
        _ => println!("  {name}: <not set>"),
    }
}

fn which_command(args: &[OsString]) -> i32 {
    let Some(command) = args.first() else {
        eprintln!("usage: winux which <command>");
        return 2;
    };

    let candidates = find_candidates(command);
    if candidates.is_empty() {
        eprintln!("{}: not found", command.to_string_lossy());
        return 1;
    }

    for candidate in candidates {
        println!("{}", candidate.display());
    }
    0
}

fn run_command(args: &[OsString]) -> i32 {
    let Some(program) = args.first() else {
        eprintln!("usage: winux run <command> [args...]");
        return 2;
    };

    match Command::new(program).args(&args[1..]).status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => {
            eprintln!("failed to run {}: {error}", program.to_string_lossy());
            1
        }
    }
}

fn find_candidates(command: &OsString) -> Vec<PathBuf> {
    let command_path = PathBuf::from(command.as_os_str());
    if has_path_separator(&command_path) {
        return if is_runnable_file(&command_path) {
            vec![command_path]
        } else {
            Vec::new()
        };
    }

    let Some(path_value) = env::var_os("PATH") else {
        return Vec::new();
    };

    let candidate_names = candidate_names(command);
    let mut seen = BTreeSet::new();
    let mut candidates = Vec::new();

    for path_entry in env::split_paths(&path_value) {
        for candidate_name in &candidate_names {
            let candidate = path_entry.join(candidate_name);
            if is_runnable_file(&candidate) && seen.insert(normalize_for_dedupe(&candidate)) {
                candidates.push(candidate);
            }
        }
    }

    candidates
}

fn candidate_names(command: &OsString) -> Vec<OsString> {
    if Path::new(command.as_os_str()).extension().is_some() {
        return vec![command.clone()];
    }

    #[cfg(windows)]
    {
        let mut names = vec![command.clone()];
        let pathext =
            env::var_os("PATHEXT").unwrap_or_else(|| OsString::from(".COM;.EXE;.BAT;.CMD"));

        for extension in env::split_paths(&pathext) {
            if let Some(extension) = extension.to_str() {
                let mut name = command.to_string_lossy().to_string();
                name.push_str(extension);
                names.push(OsString::from(name));
            }
        }

        names
    }

    #[cfg(not(windows))]
    {
        vec![command.clone()]
    }
}

fn has_path_separator(path: &Path) -> bool {
    path.components().count() > 1
}

fn is_runnable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata().map(|metadata| metadata.permissions().mode() & 0o111 != 0).unwrap_or(false)
    }

    #[cfg(not(unix))]
    {
        true
    }
}

fn normalize_for_dedupe(path: &Path) -> String {
    let value = path.to_string_lossy().to_string();
    #[cfg(windows)]
    {
        value.to_ascii_lowercase()
    }
    #[cfg(not(windows))]
    {
        value
    }
}
