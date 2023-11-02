use std::fs::DirEntry;
use std::path::Path;

///迭代目录
// fn get_file_path(path:&str, result:&mut Vec<&str>){
// fn get_file_path<T:Into<PathBuf>>(path:T, result:&mut Vec<&str>){
// fn get_file_path<T:Into<PathBuf>>(path:T, result:&mut Vec<DirEntry>){
pub fn get_file_path<T>(path:T, result:&mut Vec<DirEntry>)
// where T:Into<PathBuf> + AsRef<Path>
// where T:Into<PathBuf>
    where T:AsRef<Path> //标准库的fs::create_dir_all()用的这个
{
    let dir = path.as_ref();
    // let dir = PathBuf::from(path);
    for entry in dir.read_dir().expect("read_dir call failed"){
        if let Ok(entry) = entry {
            if entry.metadata().expect("Couldn't get metadata").is_dir(){
                get_file_path(entry.path(), result)
            }else {
                if let Some(extension) = entry.path().extension(){
                    if extension=="pdf" {
                        result.push(entry)
                    }
                }
            }
        }
    }
}