extern crate core;

mod limit;
mod server;
mod tc;

use crate::server::Server;

fn main() -> std::io::Result<()> {
    let server = Server::new("0.0.0.0:65432");
    server.listen()?;

    Ok(())
}
