use crate::utils::config::arg::ServiceConfig;
use serde_json::Value;
use std::collections::HashMap;
use std::process;
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::OnceCell;
#[derive(Debug)]
pub struct ScriptMapper {
    arg_mapper: HashMap<String, Vec<String>>,
    exec_mapper: HashMap<String, String>,
}
static LOADED_CONFIG: OnceCell<ScriptMapper> = OnceCell::const_new();
impl ScriptMapper {
    fn new() -> ScriptMapper {
        Self {
            arg_mapper: HashMap::new(),
            exec_mapper: HashMap::new(),
        }
    }
    fn load_args(&mut self, key: &str, split: Vec<String>) {
        self.exec_mapper.insert(key.to_string(), split[0].clone());
        self.arg_mapper.insert(
            key.to_string(),
            split.into_iter().skip(1).collect::<Vec<_>>(),
        );
    }
    async fn from_file(file_path: &str) -> ScriptMapper {
        if !create_if_not_exist(file_path).await {
            println!("File Not Exist,Created Config File");
            process::exit(0);
        }
        let mut mp = ScriptMapper::new();
        let file = File::options()
            .read(true)
            .open(file_path)
            .await
            .expect("Fail On Open ScriptMapper");
        let content = fs::read_to_string(file_path)
            .await
            .expect("Fail On Read ScriptMapper");
        drop(file);
        let json_data: Value = serde_json::from_str(&content).expect("Fail On Parse ScriptMapper");
        if let Value::Object(map) = json_data {
            for (key, value) in map {
                if let Some(v) = value.as_str() {
                    let args = v
                        .split_whitespace()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>();
                    if args.len() == 0 {
                        println!("Value Is Not Empty ,Skipping")
                    }
                    mp.load_args(&key, args);
                } else {
                    println!("Value Is Not String ,Skipping")
                }
            }
        }
        mp
    }
    pub async fn get() -> &'static ScriptMapper {
        LOADED_CONFIG
            .get_or_init(|| async {
                ScriptMapper::from_file(&ServiceConfig::get().await.script_mapper_file).await
            })
            .await
    }
    pub async fn init() {
        LOADED_CONFIG
            .set(ScriptMapper::from_file(&ServiceConfig::get().await.script_mapper_file).await)
            .expect("Error on Load ScriptMapper")
    }
}
async fn create_if_not_exist(file_path: &str) -> bool {
    if !std::path::Path::new(file_path).exists() {
        let mut file = File::create(file_path)
            .await
            .expect("Create Config With err");
        file.write_all("{}".as_bytes())
            .await
            .expect("Create Config Write With err");
        return false;
    }
    true
}
