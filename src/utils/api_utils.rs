use std::fmt::{Error, Result};
use std::{collections::HashSet, sync::Arc};

use crate::apps::system;
use crate::database::{db_conn, DB};
use crate::utils;
use once_cell::sync::Lazy;
use sea_orm_casbin_adapter::casbin::CoreApi;
use tokio::sync::Mutex;
use tracing::info;

pub static ALL_APIS: Lazy<Arc<Mutex<HashSet<String>>>> = Lazy::new(|| {
    let apis: HashSet<String> = HashSet::new();
    Arc::new(Mutex::new(apis))
});

pub async fn init_all_api() {
    let db = DB.get_or_init(db_conn).await;
    let res = system::get_all_sys_menu(db).await;
    let mut apis = ALL_APIS.lock().await;
    match res {
        Ok(menus) => {
            for menu in menus {
                apis.insert(menu.api.clone());
            }
        }
        Err(e) => {
            info!("初始化时获取路由API失败:{:#?}", e)
        }
    }
}

pub async fn add_api(api: &str) {
    let mut apis = ALL_APIS.lock().await;
    apis.insert(api.to_string());
}

pub async fn remove_api(api: &str) {
    let mut apis = ALL_APIS.lock().await;
    apis.remove(api);
}

pub async fn is_in(api: &str) -> bool {
    let apis = ALL_APIS.lock().await;
    apis.contains(api)
}
pub async fn check_api_permission(api: &str, method: &str) -> bool {
    let e = utils::get_enforcer(false).await;
    match e.enforce((api, method)) {
        Ok(_) => true,
        Err(err) => {
            info!("检查权限失败:{:#?}", err);
            false
        }
    }
}
