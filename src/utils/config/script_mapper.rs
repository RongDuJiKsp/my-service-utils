use crate::utils::config::arg::ServiceConfig;
use serde_json::Value;
use std::collections::HashMap;
use std::process;
use std::process::Output;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::OnceCell;
const EXEC_PARAM: &str = "exec";
const QUERY_PARAM: &str = "query";
#[derive(Debug)]
pub struct ScriptMapper {
    exec_mapper: HashMap<String, String>,
    arg_mapper: HashMap<String, Vec<String>>,
    query_mapper: HashMap<String, Vec<String>>,
}
static LOADED_CONFIG: OnceCell<ScriptMapper> = OnceCell::const_new();
impl ScriptMapper {
    fn new() -> ScriptMapper {
        Self {
            arg_mapper: HashMap::new(),
            exec_mapper: HashMap::new(),
            query_mapper: HashMap::new(),
        }
    }
    fn load_args(&mut self, key: &str, split: Vec<String>) {
        self.exec_mapper.insert(key.to_string(), split[0].clone());
        self.arg_mapper.insert(
            key.to_string(),
            split.into_iter().skip(1).collect::<Vec<_>>(),
        );
    }
    fn load_query(&mut self, key: &str, split: Vec<String>) {
        self.query_mapper.insert(key.to_string(), split);
    }
    fn write_query(&self, exec: &str, query_data: &HashMap<String, String>) -> Vec<String> {
        let def = String::from("");
        self.arg_mapper[exec]
            .iter()
            .map(|arg| {
                self.query_mapper[exec]
                    .iter()
                    .fold(arg.clone(), |before, query| {
                        before.replace(&format!("?({})", query), &query_data.get(query).get_or_insert(&def))
                    })
            })
            .collect()
    }
    async fn from_json(json: &str) -> ScriptMapper {
        let mut mp = ScriptMapper::new();
        let cfg = ServiceConfig::get().await;
        let json_data: Value = serde_json::from_str(&json).expect("Fail On Parse ScriptMapper");
        let map = match json_data.as_object() {
            None => {
                println!("File is Not Json Object");
                return mp;
            }
            Some(e) => e,
        };
        for (command_key, commend_config) in map {
            let commend_config = match commend_config.as_object() {
                None => {
                    println!("Value Is Not Object ,Skipping");
                    continue;
                }
                Some(e) => e,
            };
            let query = match commend_config.get(QUERY_PARAM) {
                None => {
                    println!("QUERY_PARAM Not Exist,Skipping");
                    continue;
                }
                Some(e) => e,
            };
            let query_str = match query.as_str() {
                None => {
                    println!("QUERY_PARAM Not String,Skipping");
                    continue;
                }
                Some(e) => e,
            };
            let exec = match commend_config.get(EXEC_PARAM) {
                None => {
                    println!("EXEC_PARAM Not Exist,Skipping");
                    continue;
                }
                Some(e) => e,
            };
            let exec_str = match exec.as_str() {
                None => {
                    println!("EXEC_PARAM Not String,Skipping");
                    continue;
                }
                Some(e) => e,
            };
            let exec_args = exec_str.split_whitespace().map(|x| x.to_string()).collect();
            if cfg.script_mapper_load_env {
                mp.load_args(&command_key, load_env(exec_args, cfg));
            } else {
                mp.load_args(&command_key, exec_args);
            }
            mp.load_query(
                &command_key,
                query_str
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect(),
            );
        }
        mp
    }
    async fn from_file(file_path: &str) -> ScriptMapper {
        if !create_if_not_exist(file_path).await {
            println!("File Not Exist,Created Config File");
            process::exit(0);
        }
        let file = File::options()
            .read(true)
            .open(file_path)
            .await
            .expect("Fail On Open ScriptMapper");
        let content = fs::read_to_string(file_path)
            .await
            .expect("Fail On Read ScriptMapper");
        drop(file);
        ScriptMapper::from_json(&content).await
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
    pub fn exist_all_query(&self, exec: &str, query: &HashMap<String, String>) -> bool {
        self.query_mapper[exec].iter().filter(|x| !query.contains_key(*x)).count() == 0
    }
    pub fn exist(&self, exec: &str) -> bool {
        self.exec_mapper.contains_key(exec)
    }
    pub async fn wait_exec(
        &self,
        exec: &str,
        query: &HashMap<String, String>,
    ) -> anyhow::Result<String> {
        if !self.exist(exec) {
            anyhow::bail!("Command {} Not Exist", exec);
        }
        let Output {
            stdout,
            stderr,
            status,
            ..
        } = Command::new(&self.exec_mapper[exec])
            .args(&self.write_query(exec, query))
            .output()
            .await?;
        let out_str = String::from_utf8(stdout)?;
        let err_str = String::from_utf8(stderr)?;
        let status = status.to_string();
        Ok(format!(
            "Script Exit With {} \n Output: {} \n Error : {} \n",
            status, out_str, err_str
        ))
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
const ENV_SCRIPT_PATH: &str = "&[SCRIPT_PATH]";
fn load_env(from: Vec<String>, cfg: &ServiceConfig) -> Vec<String> {
    from.into_iter()
        .map(|x| x.replace(ENV_SCRIPT_PATH, &cfg.script_dir))
        .collect()
}
