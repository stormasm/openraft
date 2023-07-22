use clap::Parser;
use raft_kv_memstore::start_example_raft_node;

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    #[clap(long)]
    pub id: u64,

    #[clap(long)]
    pub http_addr: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    console_subscriber::init();
    // Parse the parameters passed by arguments.
    let options = Opt::parse();

    start_example_raft_node(options.id, options.http_addr).await
}
