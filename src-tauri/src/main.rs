// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod test;
mod pdf;
mod database;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use database::service::service::{import_all_pdf, query_all_pdf};

use std::process::Command;
use serde::{Deserialize, Serialize};
use tauri::{generate_handler};
use uuid::Uuid;
#[tokio::main]
async fn main() {
  database::init_database().await;
  tauri::Builder::default()
      .invoke_handler(generate_handler![
        greet,
        greets,
        open_paper,
        import_all_pdf,
        query_all_pdf,
          read_clipboard_string,
          append_chunk_to_file
      ])
      // .invoke_handler(generate_handler![greets])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
#[tauri::command]
fn append_chunk_to_file(path: String, chunk: Vec<u8>) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| e.to_string())?;

    file.write_all(&chunk).map_err(|e| e.to_string())?;

    Ok(())
}
#[tauri::command]
fn greet(name: &str) {
  println!("收到了消息{name}");
  format!("Hello, {}!", name);
}
#[tauri::command]
fn greets(name: Vec<&str>) -> String {
  println!("收到了消息{:?}",name);
  format!("Hello, {}!", name[0])
}
#[tauri::command]
fn read_clipboard_string() -> Vec<PathBuf> {
    let files = clipboard_files::read();
    match files {
        Ok(v) => { v }
        Err(_) => { vec![] }
    }
}
// #[tauri::command]
// async fn import_all_pdf(dirs: Vec<String>) -> Vec<database::entity::papers::Model> {
//   let mut pdfs = vec![];
//   let mut papers: Vec<(OsString, PathBuf)> = vec![];
//   for dir in dirs {
//     get_file_path(dir,&mut pdfs)
//   }
//   for pdf in pdfs {
//     papers.push((pdf.file_name(),pdf.path()))
//   }
//   database::database::write_all_pdf(papers).await;
//   database::database::query_all_pdf().await
// }
// #[tauri::command]
// fn get_all_pdf(dirs: Vec<&str>) -> Vec<Paper> {
//   let mut pdfs = vec![];
//   let mut papers = vec![];
//   for dir in dirs {
//     get_file_path(dir,&mut pdfs)
//   }
//   for pdf in pdfs {
//     papers.push(Paper::new(
//       pdf.file_name().to_str().unwrap().into(),
//       pdf.path().to_str().unwrap().into(),
//       "无".into(),
//       "无".into()))
//   }
//   println!("{:?}",papers);
//   papers
// }
// ///迭代目录
// // fn get_file_path(path:&str, result:&mut Vec<&str>){
// // fn get_file_path<T:Into<PathBuf>>(path:T, result:&mut Vec<&str>){
// // fn get_file_path<T:Into<PathBuf>>(path:T, result:&mut Vec<DirEntry>){
// fn get_file_path<T>(path:T, result:&mut Vec<DirEntry>)
// // where T:Into<PathBuf> + AsRef<Path>
// // where T:Into<PathBuf>
// where T:AsRef<Path> //标准库用的这个fs::create_dir_all()
// {
//   let dir = path.as_ref();
//   // let dir = PathBuf::from(path);
//   for entry in dir.read_dir().expect("read_dir call failed"){
//     if let Ok(entry) = entry {
//       if entry.metadata().expect("Couldn't get metadata").is_dir(){
//         get_file_path(entry.path(), result)
//       }else {
//         if let Some(extension) = entry.path().extension(){
//           if extension=="pdf" {
//             result.push(entry)
//           }
//         }
//       }
//     }
//   }
// }
#[tauri::command]
fn open_paper(path:&str)->bool{
  let reader = "D:\\知云\\ZhiyunTranslator\\ZhiYunTranslator.exe";
  Command::new(reader).arg(path).spawn().map_or(false,|_| {
    true
  })
}
// #[derive(Debug,Serialize,Deserialize)]
// struct Paper{
//   pub id:Uuid,
//   pub name:String,
//   pub path:String,
//   pub abstracts:String,
//   pub content:String,
// }
//
// impl Paper {
//   fn new(name:String,path:String,abstracts:String,content:String)->Self{
//     Paper{
//       id:Uuid::new_v4(),
//       name,
//       path,
//       abstracts,
//       content
//     }
//   }
// }
