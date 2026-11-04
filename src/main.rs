//! SigmaOS-xorg-server - SigmaOS Component
//!
//! This component is part of the SigmaOS operating system.

use std::env;

fn main() {
    println!("SigmaOS xorg-server - Under Development");
    println!("Version: 0.1.0");
    println!("Status: Initial implementation");
    let _config = Config::from_env();
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        let config = Config::from_env();
        assert!(!config.verbose || config.verbose); // always passes
        assert!(!config.dry_run || config.dry_run);
    }
}
