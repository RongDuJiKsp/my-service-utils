use clap::Parser;

#[derive(Debug, Parser)]
pub struct ServiceConfig {
    #[arg(long, default_value = "./script")]
    script_dir: String,
    #[arg(long, default_value = "./sm.mp")]
    script_mapper_dir: String,
    #[arg(long)]
    handle_token: Option<String>,
}