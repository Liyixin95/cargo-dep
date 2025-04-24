use cargo_metadata::Metadata;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Default)]
enum Command {
    #[default]
    List,
    Feature {
        #[clap(short, long = "crate")]
        crates: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let cmd = cli.command.unwrap_or_default();

    let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();

    match cmd {
        Command::List => list(metadata),
        Command::Feature { crates } => features(crates, metadata),
    }
}

fn list(metadata: Metadata) {
    metadata.packages.into_iter().for_each(|p| {
        println!("{}", p.manifest_path.ancestors().nth(1).unwrap());
    })
}

fn features(crates: String, metadata: Metadata) {
    let target = metadata
        .packages
        .into_iter()
        .flat_map(|p| p.dependencies)
        .find(|dep| dep.name == crates);

    if let Some(target) = target {
        println!("{}", target.features.join("\n"));
    } else {
        println!("{} not found", crates);
    }
}
