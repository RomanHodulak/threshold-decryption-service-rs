mod public_key;

use crate::public_key::PublicKey;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::Parser;
use clio::ClioPath;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The `threshold` number of private key shares needed for decryption.
    #[arg(short, long, default_value_t = 3)]
    threshold: u8,

    /// The file path of the `public_key` to share on an endpoint.
    #[clap(long, short, value_parser = clap::value_parser!(ClioPath).exists().is_file(), default_value = "public_key.pem")]
    public_key: ClioPath,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    env_logger::init();
    let public_key = PublicKey::new(args.public_key.to_path_buf());

    HttpServer::new(move || {
        let logger = Logger::new("%a %{User-Agent}i");

        App::new()
            .service(public_key::public_key)
            .app_data(Data::new(public_key.clone()))
            .wrap(logger)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
