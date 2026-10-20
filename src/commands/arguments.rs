/*!
 * Stores the arguments for commands.
 */
use clap::Parser;
use super::structures::enums::r#type::CommandType;

/// Stores the arguments for commands.
#[derive(Parser, Debug)]
#[command(name = "cold-steel", about = "A visual application to replace 'cargo test' and 'cargo bench'.", long_about = "A visual application to replace 'cargo test' and 'cargo bench'.")]
pub struct Arguments
{
    /// The type of command to run.
    #[command(subcommand)]
    pub command: CommandType
}