use std::process::Command;

use anyhow::{anyhow, Result};
use log::{debug, trace};

pub fn check_command_exists(command: &str) -> Result<()> {
    debug!("Checking if {command} exists");
    trace!("check_command_exists({command})");

    trace!("which {command}");
    match Command::new("which")
        .arg(command)
        .output()?
        .status
        .success()
    {
        true => {
            debug!("Command {command} does exist");
            Ok(())
        }
        false => Err(anyhow!(
            "Command {command} doesn't exist and is required to build the image"
        )),
    }
}