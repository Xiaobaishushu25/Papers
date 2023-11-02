use std::ffi::OsString;
use std::path::PathBuf;
use crate::database::entity::prelude::Paper;
use super::super::utils::get_file_path;
use super::super::dao::dao::{insert_all_pdf, select_all_pdf};

#[tauri::command]
pub async fn import_all_pdf(dirs: Vec<String>) -> Vec<Paper> {
    let mut pdfs = vec![];
    let mut papers: Vec<(OsString, PathBuf)> = vec![];
    for dir in dirs {
        get_file_path(dir,&mut pdfs)
    }
    for pdf in pdfs {
        papers.push((pdf.file_name(),pdf.path()))
    }
    insert_all_pdf(papers).await;
    select_all_pdf().await
}
#[tauri::command]
pub async fn query_all_pdf() -> Vec<Paper> {
    select_all_pdf().await
}