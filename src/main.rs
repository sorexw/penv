use std::env;
use std::fs;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let repo_url = "git@github.com:sorexw/.env.git";

    let temp_dir = "/dev/shm/nix-secrets-penv";

    let home_dir = env::var("HOME").expect("Error: HOME variable is not set!");
    let ssh_key_path = format!("{}/.ssh/id_ed25519", home_dir);

    let git_ssh_cmd = format!(
        "ssh -i {} -o IdentitiesOnly=yes -o StrictHostKeyChecking=no",
        ssh_key_path
    );

    let clone_status = Command::new("git")
        .env("GIT_SSH_COMMAND", git_ssh_cmd)
        .args(["clone", "--depth", "1", repo_url, temp_dir])
        .output()
        .expect("Failed to execute git clone process");

    if !clone_status.status.success() {
        let stderr = String::from_utf8_lossy(&clone_status.stderr);
        eprintln!("🔥 Fetch Error: Could not fetch secrets!\n{}", stderr);
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    let program = if args.len() > 1 { &args[1] } else { "dwl" };

    let mut next_command = Command::new(program);
    if args.len() > 2 {
        next_command.args(&args[2..]);
    }

    let env_path = format!("{}/env.nix", temp_dir);
    if let Ok(env_content) = fs::read_to_string(&env_path) {
        for line in env_content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let clean_key = key.trim().replace("\"", "").replace(";", "");
                let clean_val = value.trim().replace("\"", "").replace(";", "");
                next_command.env(clean_key, clean_val);
            }
        }
    } else {
        eprintln!("⚠️ Warning: env file not found inside the repo!");
    }

    let _ = fs::remove_dir_all(temp_dir);

    let err = next_command.exec();
    eprintln!("Failed to execute '{}': {}", program, err);
}
