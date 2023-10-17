use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Cursor, Write, stdout}, collections::{HashSet, HashMap}, sync::Mutex,
};

use git2::{BranchType, Repository};
use serde::{Serialize, Deserialize};
use toml::Table;
use walkdir::WalkDir;
use lazy_static::lazy_static;
lazy_static!{
    static ref MAP:Mutex<HashMap<&'static str,&'static str>> = {
        let mut map: HashMap<&'static str,&'static str> = HashMap::new();
        map.insert("https://github.com/", "https://ghproxy.com/https://github.com");
        map.insert("https://raw.githubusercontent.com", "https://ghproxy.com/https://raw.githubusercontent.com");
        Mutex::new(map)
    };
}
fn main() {
    let contents = fs::read_to_string("config.toml")
        .expect("Should have been able to read the file");
    let value = contents.parse::<Table>().unwrap();

}

#[test]
pub fn walk_file() {
    for entry in WalkDir::new("/home/light/rust_project/winget-pkgs/manifests")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        println!("查询到文件名称为 {f_name}");
        if !f_name.ends_with("installer.yaml") {
            continue;
        }
        //
        let file_path = entry.path();
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        // 创建一个临时文件用于写入替换后的内容
        let temp_file_path = format!("{}_temp", file_path.to_string_lossy());
        let mut temp_file = File::create(&temp_file_path).unwrap();

        let mut modified = false;
        for line in reader.lines().map(Result::ok) {
            if let Some(_inner) = line {
                if _inner.starts_with("  InstallerUrl: https://github.com") {
                    let replace_line = _inner.replace(
                        "  InstallerUrl: https://github.com",
                        "  InstallerUrl: https://ghproxy.com/https://github.com",
                    );
                    println!("将文本 {_inner} 替换为 {replace_line}");
                    writeln!(temp_file, "{}", replace_line).unwrap();
                    modified = true;
                } else if _inner.starts_with("  InstallerUrl: https://raw.githubusercontent.com") {
                    let replace_line = _inner.replace(
                        "  InstallerUrl: https://raw.githubusercontent.com",
                        "  InstallerUrl: https://ghproxy.com/https://raw.githubusercontent.com",
                    );
                    println!("将文本 {_inner} 替换为 {replace_line}");
                    writeln!(temp_file, "{}", replace_line).unwrap();
                    modified = true;
                } else {
                    writeln!(temp_file, "{}", _inner).unwrap();
                }
            }
        }
        // 关闭文件
        drop(temp_file);
        if modified {
            // 删除原始文件
            fs::remove_file(file_path).unwrap();

            // 重命名临时文件为原始文件名
            std::fs::rename(temp_file_path, file_path).unwrap();
        } else {
            // 删除临时文件
            fs::remove_file(temp_file_path).unwrap();
        }
    }
}

#[derive(Deserialize,Serialize,Debug)]
struct Config{
    #[serde(rename = "proxy.map")]
    map_url:HashSet<String>,
}

#[test]
fn parse_toml_file(){
    let contents = fs::read_to_string("config.toml")
        .expect("Should have been able to read the file");
    let value = contents.parse::<Table>().unwrap();
    assert_eq!(value["https://github.com/"].as_str(),Some("https://ghproxy.com/https://github.com"));
}