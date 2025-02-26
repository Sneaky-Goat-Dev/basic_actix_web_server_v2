use std::io::Error;

use basic_actix_web_server_v2::run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    run().await
}
