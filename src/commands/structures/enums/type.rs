/*!
 * Stores the types of commands that are processed.
 */
use clap::Subcommand;

/// Stores the types of commands that are processed.
#[derive(Subcommand, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType
{
    /// The 'cold-steel test' command overriding the 'cargo test' command.
    #[command(about = "The replacement for the 'cargo test' command.", long_about = "The replacement for the 'cargo test' command.")]
    Test
    {
        /// The flag to determine if the timings of tests should be included in the display.
        #[arg(short, long, help = "The flag to determine if the timings of tests should be included in the display.", long_help = "The flag to determine if the timings of tests should be included in the display.")]
        timings: bool,

        /// The flag to determine if the application should output a pretty output upon exiting.
        #[arg(short, long, help = "The flag to determine if the application should output a pretty output upon exiting.", long_help = "The flag to determine if the application should output a pretty output upon exiting.")]
        print: bool
    },

    /// The 'cold-steel bench' command overriding the 'cargo bench' command.
    #[command(about = "The replacement for the 'cargo bench' command.", long_about = "The replacement for the 'cargo bench' command.")]
    Bench
    {
        /// The flag to determine if the application should output a pretty output upon exiting.
        #[arg(short, long, help = "The flag to determine if the application should output a pretty output upon exiting.", long_help = "The flag to determine if the application should output a pretty output upon exiting.")]
        print: bool
    }
}