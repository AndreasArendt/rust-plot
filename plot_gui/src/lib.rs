use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;

pub fn launch() {
    let exe = find_gui_binary().expect("GUI binary not found");

    Command::new(exe)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to launch GUI");
}

fn find_gui_binary() -> Option<PathBuf> {
    let mut path = env::current_exe().ok()?;

    // Strip the current binary file name (e.g., "main_app")
    path.pop();

    // Windows: add `.exe`
    #[cfg(target_os = "windows")]
    path.push("plot_gui.exe");

    #[cfg(not(target_os = "windows"))]
    path.push("plot_gui");

    println!("Looking for GUI binary at: {:?}", path);

    if path.exists() {
        Some(path)
    } else {
        eprintln!("GUI binary not found at: {:?}", path);
        None
    }
}