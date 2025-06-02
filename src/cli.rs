use clap::Parser;

// create a struct CLI to hold the command line arguments
#[derive(Parser)]
#[clap(name = "demo")]
pub struct Cli {
    // the name for the event:
    #[clap(short = 'n', long = "name", default_value = "aria")]
    pub name: String,

    // the date of birth in format YYYY-MM-DD HH:MM:SS (default: current date)
    #[clap(short = 'd', long = "datetime")]
    pub datetime: Option<String>,

    // the timezone of the date (default: UTC)
    #[clap(short = 't', long = "timezone")]
    pub timezone: Option<String>,

    // allow for debug flag
    #[clap(short = 'v', long = "verbose")]
    pub debug: bool,
}
