use crate::utils::config::script_mapper::ScriptMapper;
use std::collections::HashMap;

pub async fn call_script(script: &str, query: &HashMap<String, String>) -> String {
    let s = ScriptMapper::get().await;
    if !s.exist_all_query(script, query) {
        return String::from("Query Param Not Enough!");
    }
    s.wait_exec(script, query).await.unwrap_or_else(|e| {
        format!("Exec Command With Error : {}", e)
    })
}
