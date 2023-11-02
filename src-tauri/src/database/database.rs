use sea_orm::EntityTrait;
use std::{env, fs};
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::OnceLock;
use sea_orm::{ActiveValue, ConnectionTrait, Database, DatabaseConnection, DeriveEntity, Schema};
use serde::{Deserialize, Serialize};
use crate::database::entity::papers;
use crate::database::entity::prelude::Papers;

pub static DB: OnceLock<DatabaseConnection> = OnceLock::new();
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

pub async fn query_all_pdf() -> Vec<papers::Model>{
    Papers::find().all(DB.get().expect("获取数据库连接失败")).await.unwrap()
}
pub fn update_all_path(){

}
pub async fn write_all_pdf(pdfs:Vec<(OsString,PathBuf)>){
    let mut paper_list = vec![];
    for pdf in pdfs {
        paper_list.push(papers::ActiveModel{
            id: ActiveValue::NotSet,
            name:ActiveValue::Set(pdf.0.to_string_lossy().to_string()),
            path:ActiveValue::Set(pdf.1.to_string_lossy().to_string()),
            abstracts:ActiveValue::Set(None),
            content:ActiveValue::Set(None)
        })
    }
    println!("插入{:?}",paper_list);
    papers::Entity::insert_many(paper_list).exec(DB.get().unwrap()).await.expect("插入数据失败");
    // papers::insert_many(paper_list).exec(DB.get().unwrap()).await.expect("插入数据失败");
}