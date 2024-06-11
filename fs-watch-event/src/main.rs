use notify::Result;

use fs_watch_event::{parse_config, run_loop};

fn main() -> Result<()> {
    match parse_config() {
        Ok((watch_path, backup_path)) => run_loop(&watch_path, &backup_path),
        Err(e) => {
            println!("{:}", e);
            Ok(())
        }
    }
}
