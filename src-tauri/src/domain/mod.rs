pub(crate) mod table;
pub(crate) mod mapper;

use std::process::id;
use log::LevelFilter;
use rbatis::Rbatis;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use rbatis::table_sync::{RbatisTableSync, SqliteTableSync};
use rbdc::db::Driver;
use rbdc_sqlite::driver::SqliteDriver;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};
use crate::domain::table::live_info::LiveInfo;

// pub static mut RB: Lazy<Rbatis> = Lazy::new(|| init_db());
pub static mut RB: OnceCell<Rbatis> = OnceCell::new();

/// make a sqlite-rbatis
pub async fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(SqliteDriver {}, "live.db")
        .unwrap();

    // ------------sync tables------------
    // 判断 live.db 是否存在
    if !std::path::Path::new("live.db").exists() {
        // 如果不存在，创建 live.db
        let mut sql = "CREATE TABLE live_info (
  id INTEGER NOT NULL,
  name TEXT,
  status TEXT,
  create_time DATE,
  room_id text,
  site_name TEXT,
  site_url TEXT,
  PRIMARY KEY (id)
);";
        fast_log::LOGGER.set_level(LevelFilter::Off);
        let _ = rb.exec(&sql, vec![]).await;
        fast_log::LOGGER.set_level(LevelFilter::Info);
    }

    // // ------------sync tables------------
    // use rbatis::rbdc::db::Driver;
    // use rbatis::table_sync::{RbatisTableSync, SqliteTableSync};
    // let mut s = RbatisTableSync::new();
    // let driver = SqliteDriver {};
    // s.insert(driver.name().to_string(), Box::new(SqliteTableSync {}));
    // fast_log::LOGGER.set_level(LevelFilter::Off);
    // s.sync(
    //     driver.name(),
    //     rb.acquire().await.unwrap(),
    //     &BizActivity {
    //         id: None,
    //         name: None,
    //         pc_link: None,
    //         h5_link: None,
    //         pc_banner_img: None,
    //         h5_banner_img: None,
    //         sort: None,
    //         status: None,
    //         remark: None,
    //         create_time: None,
    //         version: None,
    //         delete_flag: None,
    //     },
    // )
    // .await
    // .unwrap();
    // fast_log::LOGGER.set_level(LevelFilter::Info);
    // // ------------sync tables end------------

    // ------------create tables way 2------------
    // let mut f = File::open("example/table_sqlite.sql").unwrap();
    // let mut sql = String::new();
    // f.read_to_string(&mut sql).unwrap();
    // fast_log::LOGGER.set_level(LevelFilter::Off);
    // let _ = rb.exec(&sql, vec![]).await;
    // fast_log::LOGGER.set_level(LevelFilter::Info);
    // ------------create tables way 2 end------------

    return rb;
}