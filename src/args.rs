use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// SocketAddrs to bind to for SMTP. https://doc.rust-lang.org/stable/std/net/trait.ToSocketAddrs.html#tymethod.to_socket_addrs
    #[arg(long, env, default_value = "localhost:2525")]
    pub bind_smtp_addrs: String,

    /// SocketAddrs to bind to for HTTP. https://doc.rust-lang.org/stable/std/net/trait.ToSocketAddrs.html#tymethod.to_socket_addrs
    #[arg(long, env, default_value = "localhost:8080")]
    pub bind_http_addrs: String,
}
