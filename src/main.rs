fn main() {
    let cmd = cargo_metadata::MetadataCommand::new();
    let metadata = cmd.exec().unwrap();

    metadata.packages.into_iter().for_each(|p| {
        print!("{} ", p.manifest_path.ancestors().nth(1).unwrap());
    })
}
