use clap::Parser;
use tokio::sync::OnceCell;

#[derive(Debug, Parser)]
pub struct ServiceConfig {
    #[arg(long, default_value = "0.0.0.0:80")]
    pub addr: String,
    #[arg(long, default_value = "./script")]
    pub script_dir: String,
    #[arg(long, default_value = "./sm.json")]
    pub script_mapper_file: String,
    //Envs: "&[SCRIPT_PATH]"
    #[arg(long)]
    pub script_mapper_load_env: bool,
    #[arg(long)]
    pub handle_token: Option<String>,
}
pub static SERVICE_CONFIG: OnceCell<ServiceConfig> = OnceCell::const_new();
impl ServiceConfig {
    pub async fn get() -> &'static ServiceConfig {
        SERVICE_CONFIG
            .get_or_init(|| async { ServiceConfig::parse() })
            .await
    }
    pub async fn init() {
        SERVICE_CONFIG
            .set(ServiceConfig::parse())
            .expect("Load Service Config Failed");
    }
}
