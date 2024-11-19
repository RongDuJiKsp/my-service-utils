use std::collections::HashMap;
use crate::utils::config::script_mapper::ScriptMapper;

pub async fn call_script(script: &str, query: &HashMap<String, String>) -> String {
    let s = ScriptMapper::get().await;
    if !s.exist_all_query(script, query) {
        return String::from("Query Param Not Enough!");
    }
    match s.wait_exec(script, query).await {
        Ok(o) => {
            o
        }
        Err(e) => {
            format!("Exec Command With Error : {}", e)
        }
    }
}
