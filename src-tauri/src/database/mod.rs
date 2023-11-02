use std::{env, fs};
use std::sync::OnceLock;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};
use crate::database::entity::prelude::Papers;

pub mod database;
pub mod entity;
pub mod service;
mod dao;
mod utils;

static DB: OnceLock<DatabaseConnection> = OnceLock::new();
async fn create_database()->DatabaseConnection{
    println!("数据库文件不存在，开始创建");
    //?mode=rwc表示以读写创建模式打开数据库文件，如果文件不存在则创建一个新的文件
    let database_path = env::current_dir().unwrap().join("database"); // 构建数据库文件夹路径
    fs::create_dir_all(&database_path).unwrap(); // 创建文件夹及其父文件夹，如果不存在的话
    let url = format!("sqlite:{}/data.db?mode=rwc", database_path.to_string_lossy());
    let connection = Database::connect(url).await.expect("数据库连接失败");
    let backend = connection.get_database_backend();
    println!("{:?}",backend);
    let schema = Schema::new(backend);
    connection.execute(backend.build(&schema.create_table_from_entity(Papers))).await.expect("创建数据库失败");
    connection
}
pub async fn init_database(){
    let connection = match env::current_dir().unwrap().join("database").join("data.db").exists(){
        true =>{
            let url = format!("sqlite:{}/data.DB?mode=rwc", env::current_dir().unwrap().join("database").to_string_lossy());
            Database::connect(url).await.expect("数据库连接失败")
        }
        false =>{
            create_database().await
        }
    };
    DB.get_or_init(||connection);
    // DB.get_or_init(||-> DatabaseConnection {
    //     tokio::task::block_in_place(async {
    //         match env::current_dir().unwrap().join("database").join("data.db").exists(){
    //             true =>{
    //                 let url = format!("sqlite:{}/data.DB?mode=rwc", env::current_dir().unwrap().join("database").to_string_lossy());
    //                 Database::connect(url).await.expect("数据库连接失败")
    //             }
    //             false =>{
    //                 create_database().await
    //             }
    //         }
    //     })
    // } );
}