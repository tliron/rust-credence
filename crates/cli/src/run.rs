use super::{cli::*, errors::*};

use {clap::*, kutil_cli::log::*, tokio::runtime::*};

/// Run.
pub fn run() -> Result<(), MainError> {
    let cli = CLI::parse();

    if cli.journald {
        initialize_tracing_journald(cli.verbose + 2)?;
    } else if !cli.quiet {
        initialize_tracing(cli.verbose + 2, cli.log_path.as_ref())?;
    }

    match &cli.subcommand {
        None => {
            let tokio = Builder::new_multi_thread().enable_all().build()?;
            tokio.block_on(cli.start())?;
        }

        Some(subcommand) => match subcommand {
            SubCommand::Version(version) => version.run::<CLI>(),
            SubCommand::Completion(completion) => completion.run::<CLI>(),
        },
    }

    Ok(())
}
