use super::{cli::*, errors::*};

use {clap::*, kutil::cli::log::*, tokio::runtime::*};

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

        Some(SubCommand::Version(version)) => version.run::<CLI>(),
        Some(SubCommand::Completion(completion)) => completion.run::<CLI>(),
        Some(SubCommand::Manual(manual)) => manual.run::<CLI>()?,
    }

    Ok(())
}
