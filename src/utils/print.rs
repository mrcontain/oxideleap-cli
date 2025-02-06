use anyhow::Result;
use std::io::Write;
pub fn print_with_anyoutput<T: Write>(cli_header: &str, output: &mut T) -> Result<()> {
    output.write_all(cli_header.as_bytes())?;
    output.flush()?;
    Ok(())
}
