use {
    clap::{builder::*, *},
    kutil_cli::clap::*,
    std::path::*,
};

// https://docs.rs/clap/latest/clap/_derive/index.html

//
// CLI
//

/// An unfussy web server designed for straightforward authoring and scalable performance
#[derive(Parser)]
#[command(
    name = "credence",
    version,
    propagate_version = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    styles = clap_styles())
]
pub struct CLI {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    /// site assets path
    #[arg(verbatim_doc_comment)]
    pub assets_paths: Vec<PathBuf>,

    /// shutdown grace period in seconds
    #[arg(long = "grace-period", short = 'g', verbatim_doc_comment, default_value_t = 10)]
    pub grace_period: u64,

    /// suppress console output
    #[arg(long, short = 'q', verbatim_doc_comment)]
    pub quiet: bool,

    /// add a log verbosity level;
    /// can be used 3 times
    #[arg(long, short, verbatim_doc_comment, action = ArgAction::Count)]
    pub verbose: u8,

    /// log to file path;
    /// defaults to stderr
    #[arg(long, long = "log", short = 'l', verbatim_doc_comment)]
    pub log_path: Option<PathBuf>,

    /// log to journald;
    /// when true ignores --log
    #[arg(long = "journald", short = 'j', verbatim_doc_comment)]
    pub journald: bool,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// SubCommands
//

#[derive(Subcommand)]
#[command()]
pub enum SubCommand {
    /// show the version of credence
    Version(Version),

    /// output the shell auto-completion script
    Completion(Completion),

    /// output the manual pages (in the troff format)
    Manual(Manual),
}
