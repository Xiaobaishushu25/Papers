use sea_orm::{ActiveModelTrait, EntityTrait};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::OnceLock;
use sea_orm::{ActiveValue};
use sea_orm::ActiveValue::Set;
use crate::database::entity::papers;
use crate::database::entity::prelude::{Paper, Papers};
use super::super::DB;

pub static EXIST:OnceLock<HashMap<&str,&str>>= OnceLock::new();
pub static mut PAPERS:OnceLock<Vec<Paper>>= OnceLock::new();
pub async fn select_all_pdf() -> Vec<Paper>{
    let papers = Papers::find().all(DB.get().expect("获取数据库连接失败")).await.unwrap();
    // EXIST.get_or_init(||{
    //     HashMap::new()
    // });
    unsafe {
        PAPERS.get_or_init(||{Vec::new()});
        let x = PAPERS.get_mut().unwrap();
        x.clear();
        x.append(&mut papers.clone())
    }
    // papers.iter().for_each(|paper|{
    //     EXIST.get().unwrap().insert(&paper.name,&paper.path);
    // });
    papers
}
pub fn update_all_path(){

}
pub async fn insert_all_pdf(pdfs:Vec<(OsString, PathBuf)>){
    let mut paper_list = vec![];
    for pdf in pdfs {
        unsafe {
            if let Some(paper) = PAPERS.get().unwrap().iter().find(|paper|{
                paper.name==pdf.0.to_string_lossy()
            }){
                if paper.path!=pdf.1.to_string_lossy() {
                    let mut paper :papers::ActiveModel= (*paper).clone().into();
                    paper.path = Set(pdf.1.to_string_lossy().to_string());
                    paper.update(DB.get().unwrap()).await.expect("更新失败");
                }
            }else {
                paper_list.push(papers::ActiveModel{
                    id: ActiveValue::NotSet,
                    name:ActiveValue::Set(pdf.0.to_string_lossy().to_string()),
                    path:ActiveValue::Set(pdf.1.to_string_lossy().to_string()),
                    abstracts:ActiveValue::Set(None),
                    content:ActiveValue::Set(None)
                })
            }
        }
    }
    println!("插入{:?}",paper_list);
    papers::Entity::insert_many(paper_list).exec(DB.get().unwrap()).await.expect("插入数据失败");
}