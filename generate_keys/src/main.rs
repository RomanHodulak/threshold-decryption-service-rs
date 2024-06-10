use clap::Parser;
use clio::ClioPath;
use itertools::Itertools;
use std::fs::File;

mod generate;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The `threshold` number of `shares` needed for decryption.
    ///
    /// It must be true that `threshold` <= `shares`.
    #[arg(short, long, default_value_t = 3)]
    threshold: u8,

    /// The total number of `shares` to split the private key to.
    #[arg(short, long, default_value_t = 5)]
    shares: u8,

    /// The file path of the private `key` to generate the `shares` for.
    #[clap(long, short, value_parser = clap::value_parser!(ClioPath), default_value = "public_key.pem")]
    key: ClioPath,

    /// The prefix of the file path for the shares to generate into.
    ///
    /// For example, with the provided value being "share_", the generated file names would become
    /// "share_1.pem", "share_2.pem", ...
    #[clap(long, short, value_parser = clap::value_parser!(ClioPath), default_value = "share_")]
    output: ClioPath,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let private_key = args.key.open()?;

    let (shares, mut errors): (Vec<_>, Vec<_>) = (1..=args.shares)
        .into_iter()
        .map(|i| {
            let file_name = format!(
                "{}{i}.pem",
                args.output
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            );

            File::create(args.output.with_file_name(file_name))
        })
        .partition_result();

    if let Some(error) = errors.pop() {
        return Err(error.into());
    }

    generate::generate(private_key, shares, args.threshold)
}
