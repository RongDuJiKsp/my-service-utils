use axum::extract::Query;
use axum::routing::{get, Router};
use serde::Deserialize;
use crate::service::script_call;
use crate::utils::safe_check;

pub fn route() -> Router {
    Router::new().route("/script/qiatongxue", get(qia_tx_sn))
}
#[derive(Debug, Deserialize)]
struct QiaTongXueShaoNianParam {
    token: String,
    uuid: String,
}


async fn qia_tx_sn(Query(params): Query<QiaTongXueShaoNianParam>) -> String {
    if !safe_check::token::check_token(&params.token) {
        return format!("token :{} is not right", &params.token);
    }
    match script_call::qia_tong_xue_shao_nian(&params.token).await {
        Ok(print) => {
            format!("Script Finish: \n {}", &print)
        }
        Err(e) => {
            format!("Script Finish With error: \n {}", &e)
        }
    }
}