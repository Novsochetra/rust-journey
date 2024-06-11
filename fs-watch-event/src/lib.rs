use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn parse_config() -> std::result::Result<(String, String), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        let template = "
Missing watch path and backup directory path arguments.
Example: watch YOUR_DIRECTOR_PATH YOUR_BACKUP_PATH
        ";
        Err(format!("{:}", template))
    } else if args.len() == 2 {
        let template = "
Missing backup directory path
Example: watch YOUR_DIRECTOR_PATH YOUR_BACKUP_PATH
        ";
        Err(format!("{:}", template))
    } else {
        Ok((args[1].to_owned(), args[2].to_owned()))
    }
}

pub fn run_loop(watch_path: &str, backup_path: &str) -> Result<()> {
    if !Path::new(watch_path).exists() {
        fs::create_dir_all(watch_path).expect("❌ ERROR: Unable to create watch path");
    }

    if !Path::new(backup_path).exists() {
        fs::create_dir_all(backup_path).expect("❌ ERROR: Unable to create backup path");
    }

    let backup_trash_path = Path::new(backup_path).join(".trash");
    if !backup_trash_path.exists() {
        fs::create_dir_all(backup_trash_path).unwrap_or_else(|err| {
            println!("❌ ERROR: Failed to create directory {:?}", err);
            ()
        })
    }

    // Create a channel to receive events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;

    // Watch the specified directory recursively.
    watcher.watch(Path::new(&watch_path), RecursiveMode::Recursive)?;

    // Process events received from the channel.
    loop {
        let event_timeout = rx.recv_timeout(Duration::from_secs(5));

        match event_timeout {
            Ok(event) => match event {
                Ok(event) => handle_file_event(event, &watch_path, &backup_path),
                Err(_err) => {
                    println!("❌ ERROR: Failed to receive event");
                }
            },
            Err(_) => {
                // println!("No event received in duration 5 seconds ({:?})", err);
            }
        }
    }
}

pub fn handle_file_event(event: Event, watch_path: &str, backup_path: &str) {
    let backup_trash_path = Path::new(backup_path).join(".trash");

    match event.kind {
        EventKind::Modify(_) => {
            let path = &event.paths[0];

            // if the path doesn't exist mean the file is might be delete / removed
            if !path.exists() {
                handle_remove_event(event, watch_path, backup_path, &backup_trash_path);
            } else {
                if let Some(_) = path.file_name() {
                    let new_path = path.to_str().unwrap().replace(watch_path, backup_path);
                    println!("TRY: {:?}", new_path);

                    if path.is_file() {
                        match fs::copy(&path, &new_path) {
                            Ok(_) => {
                                println!("✅ Backup created for {:?} as {:?}", path, backup_path);
                            }
                            Err(e) => {
                                println!("❌ ERROR: Failed to copy. {:?}", e);
                            }
                        }
                    } else {
                        match fs::create_dir_all(&new_path) {
                            Ok(_) => {
                                println!("✅ Backup created for {:?} as {:?}", path, backup_path);
                            }
                            Err(e) => {
                                println!("❌ ERROR: Failed to copy. {:?}", e);
                            }
                        }
                    }
                }
            }
        }
        EventKind::Remove(_) => {
            handle_remove_event(event, watch_path, backup_path, &backup_trash_path);
        }
        _ => {
            println!("Other event: {:?}", event.paths);
        }
    }
}

pub fn handle_remove_event(
    event: Event,
    _watch_path: &str,
    backup_path: &str,
    backup_trash_path: &Path,
) {
    let path = &event.paths[0];

    let source = Path::new(backup_path).join(path.file_name().unwrap());
    let dest = backup_trash_path.join(path.file_name().unwrap());

    if path.is_dir() {
        move_file_or_folder(&source, &backup_trash_path);
    } else {
        if dest.exists() {
            let new_filename = format!(
                "{} ({}).{}",
                dest.with_extension("").to_str().unwrap(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                dest.extension().unwrap().to_str().unwrap()
            );

            let new_dest_path = Path::new(&new_filename);

            move_file_or_folder(&source, &new_dest_path);
        } else {
            move_file_or_folder(&source, &dest);
        }
    }
}

// TODO: mighe be extract to helper
fn move_file_or_folder(source: &PathBuf, dest: &Path) {
    match fs::rename(&source, &dest) {
        Ok(_) => {
            println!("✅ Removed from: {:?}", source);
        }
        Err(err) => {
            println!(
                "❌ ERROR: Failed to move {:?} to {:?} {:?}",
                source, dest, err
            );
        }
    }
}
