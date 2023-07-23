use clap::Parser;
use raft_kv_rocksdb::start_example_raft_node;

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    #[clap(long)]
    pub id: u64,

    #[clap(long)]
    pub http_addr: String,

    #[clap(long)]
    pub rpc_addr: String,
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    console_subscriber::init();
    // Parse the parameters passed by arguments.
    let options = Opt::parse();

    start_example_raft_node(
        options.id,
        format!("{}.db", options.rpc_addr),
        options.http_addr,
        options.rpc_addr,
    )
    .await
}
