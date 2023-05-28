use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// SocketAddrs to bind to. https://doc.rust-lang.org/stable/std/net/trait.ToSocketAddrs.html#tymethod.to_socket_addrs
    #[arg(short, long, env, default_value = "localhost:2525")]
    pub addrs: String,
}
