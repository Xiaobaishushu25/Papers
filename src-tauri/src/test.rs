use std::path::{ PathBuf };
use std::process::Command;

#[test]
fn iteration() {
    let exe_path = "D:\\知云\\ZhiyunTranslator\\ZhiYunTranslator.exe";
    let pdf_file1 = "F:\\科研\\论文\\英\\Towards Deep Learning Models Resistant to Adversarial.pdf";
    let pdf_file2 = "F:\\科研\\论文\\英\\One Pixel Attack for Fooling Deep Neural Networks.pdf";
    let vec1 = vec![pdf_file2,pdf_file1];
    for x in vec1 {
        let mut command = Command::new("start");
        let x1 = command.arg(exe_path).arg(x);
        println!("{:?}",x1);
        x1.spawn().unwrap();
        // command.arg(pdf_file1).spawn().unwrap();
    }
    // let mut result = Command::new(exe_path)
    //     .arg(pdf_file1)
    //     // .arg(pdf_file2)
    //     .spawn()
    //     .expect("无法打开应用程序和文件");
    //
    // let status = result.wait().expect("等待失败");
    //
    // if status.success() {
    //     println!("成功打开应用程序和文件");
    // } else {
    //     eprintln!("无法打开应用程序和文件");
    // }
    // let mut files = vec![];
    // get_file("F:\\科研\\论文".into(), &mut files);
    // println!("{:?}",files)
}
fn get_file(path:PathBuf, result:&mut Vec<String>){
    // let path = Path::new("F:\\科研\\论文");
    for entry in path.read_dir().expect("read_dir call failed"){
        if let Ok(entry) = entry {
            if entry.metadata().expect("Couldn't get metadata").is_dir(){
                get_file(entry.path(), result)
            }else {
                result.push(entry.path().to_str().unwrap().into())
            }
        }
    }
}