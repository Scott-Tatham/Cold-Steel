/*!
 * Handles the building of commands.
 */
use std::io::Error;
use std::process::{Child, Command, Stdio};

/// Builds a command.
/// # Arguments
/// * `arguments` - The arguments to pass to the command.
/// # Returns
/// `Result<Child, Error>` - The child process or an error.
/// # Errors
/// Returns an error if the command fails to build.
pub fn build_command(arguments: Vec<&str>) -> Result<Child, Error>
{
    Command::new("cargo").args(&arguments).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()
}

/// Tests for the [build_command] function.
#[cfg(test)]
mod tests
{
    use super::*;

    /// Tests the [build_command] function to assert the bench command builds.
    #[test]
    fn test_build_command_bench()
    {
        let arguments = vec!["bench", "--", "--format", "json"];
        assert!(build_command(arguments).is_ok());
    }

    /// Tests the [build_command] function to assert the test command builds.
    #[test]
    fn test_build_command_test()
    {
        let arguments = vec!["test", "--", "--format", "json"];
        assert!(build_command(arguments).is_ok());
    }
}