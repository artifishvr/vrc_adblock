use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn is_admin() -> bool {
    #[cfg(target_os = "linux")]
    {
        unsafe { libc::getuid() == 0 }
    }

    #[cfg(target_os = "windows")]
    {
        is_elevated::is_elevated()
    }
}

fn main() {
    if !cfg!(any(target_os = "windows", target_os = "linux")) {
        panic!("Only windows and linux are supported at the moment.");
    }

    if !is_admin() {
        let exe = std::env::current_exe().unwrap();
        runas::Command::new(exe)
            .gui(true) // shows UAC prompt instead of failing silently
            .status()
            .expect("failed to relaunch elevated");
        return; // exit the non-elevated instance
    }

    println!("Getting latest blocked_hosts.txt file from github...");

    let body = reqwest::blocking::get("https://raw.githubusercontent.com/artifishvr/vrc_adblock/refs/heads/main/blocked_hosts.txt").expect("Couldn't get new hosts file, github down again?")
    .text().expect("Couldn't parse text");
	let path: &Path = if cfg!(target_os = "windows") {
        Path::new("C:\\Windows\\System32\\drivers\\etc\\hosts")
    } else {
        Path::new("/etc/hosts")
    };

    let section_result = update_section(path, &body);

    match section_result {
        Ok(_) => println!("Successfully updated hosts file"),
        Err(err) => println!("Failed to update file {}", err),
    }

    println!("\nPress enter to exit...");
    io::stdout().flush().expect("What");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("yeah sure whatever it finished");
}

fn update_section(path: &Path, new_content: &str) -> io::Result<()> {
    let original = fs::read_to_string(path)?;
    let lines: Vec<&str> = original.lines().collect();

    let section_start_marker: &str = "# start of vrc_adblock section don't touch CmZ9p";
    let section_end_marker: &str = "# end of vrc_adblock section don't touch CmZ9p";

    let start_idx = lines.iter().position(|l| l.trim() == section_start_marker);
    let end_idx = lines.iter().position(|l| l.trim() == section_end_marker);

    let mut output = String::new();

    match (start_idx, end_idx) {
        (Some(start), Some(end)) if end > start => {
            // section exists
            for line in &lines[..=start] {
                output.push_str(line);
                output.push('\n');
            }
            output.push_str(new_content.trim_end());
            output.push('\n');
            for line in &lines[end..] {
                output.push_str(line);
                output.push('\n');
            }
        }
        _ => {
            // section does not exist
            output.push_str(&original);
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
            output.push_str(section_start_marker);
            output.push('\n');
            output.push_str(new_content.trim_end());
            output.push('\n');
            output.push_str(section_end_marker);
            output.push('\n');
        }
    }

    // safety checks
    if output.trim().is_empty() {
        return Err(io::Error::other(
            "refusing to write: output is empty",
        ));
    }

    if !original.trim().is_empty() && (output.len() as f64) < (original.len() as f64) * 0.5 {
        return Err(io::Error::other(
            "refusing to write: output is suspiciously smaller than the original ",
        ));
    }

    fs::write(path, output)
}
