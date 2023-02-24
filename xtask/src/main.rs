use clap::Parser;
use console::style;
use sqlx_cli::Opt;
use std::process;

// cargo invokes this binary as `cargo-sqlx sqlx <args>`
// so the parser below is defined with that in mind
#[derive(Parser, Debug)]
#[clap(bin_name = "cargo xtask")]
enum Cli {
    Sqlx(Opt),
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let Cli::Sqlx(opt) = Cli::parse();

    if let Err(error) = sqlx_cli::run(opt).await {
        println!("{} {}", style("error:").bold().red(), error);
        process::exit(1);
    }
}
