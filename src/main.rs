use {
    argh::FromArgs,
    okstd::prelude::*,
    std::{error::Error, path::PathBuf},
    tokio::fs::read_to_string,
    typefrog::compute,
};

#[derive(FromArgs)]
/// Reach new heights.
struct Typefrog {
    #[argh(subcommand)]
    /// cmd
    cmd: SubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommand {
    Convert(Convert),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Convert
#[argh(subcommand, name = "convert")]
struct Convert {
    /// in, file to read
    #[argh(option, short = 'i')]
    r#in: PathBuf,

    /// out, file to write
    #[argh(option, short = 'o')]
    out: PathBuf,
}

#[okstd::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let typefrog: Typefrog = argh::from_env();
    match typefrog.cmd {
        SubCommand::Convert(convert) => convert.execute().await,
    }
}

trait ExecutableCommand: SubCommands {
    async fn execute(&self) -> Result<(), Box<dyn Error>>;
}

impl ExecutableCommand for Convert {
    async fn execute(&self) -> Result<(), Box<dyn Error>> {
        let input = read_to_string(&self.r#in).await?;
        compute(&input).unwrap();
        Ok(())
    }
}
