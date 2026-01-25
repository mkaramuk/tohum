use std::process::Output;

use anyhow::Error;

pub fn check_exit_status(output: Output) -> Result<(), Error> {
    if !output.status.success() {
        let err_output = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::msg(err_output));
    }

    Ok(())
}
