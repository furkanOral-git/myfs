use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;


pub fn test_fs() {
    let path: &str = "./projects/TestProject";
    //create_dir_path(path)
    //remove_file(path, "test.html");
    //create_file_with_path(path, "test.html");
    //update_file_content(path, "test.html", "Furkan Oral");
    let _content = read_file(path, "test.html").unwrap();
    println!("{_content}");
}
pub fn read_file(path: &str, name: &str) -> Option<String> {
    
    let full_path: String = check_file_path_and_return_full_path(path, name);
    if full_path.is_empty() {
        return None;
    }

    let mut res: Result<File, Error> = OpenOptions::new().read(true).open(full_path);
    let _as_mut_res: Result<&mut File, &mut Error> = res.as_mut();

    match _as_mut_res {
        
        Ok(file) => {
            
            let mut buffer: String = String::new();
            let _ = file.read_to_string(&mut buffer);
            return Some(buffer);
        }
        Err(err) => {
            println!("Error Details : {err}");
            None
        }
    }
}
pub fn create_dir(path: &str) {
    let result: Result<(), Error> = std::fs::create_dir(path);
    if result.is_err() {
        let err: Error = result.err().unwrap();
        println!("The path isn't exist!\n Error details : {err}");
        return;
    }
}
pub fn create_dir_path(path: &str) {
    let result: Result<(), Error> = std::fs::create_dir_all(path);
    if result.is_err() {
        let err: Error = result.err().unwrap();
        println!("Error details : {err}");
        return;
    }
}
pub fn check_path(path: &str) -> bool {
    let result: Result<bool, Error> = Path::new(path).try_exists();

    if result.is_err() {
        let err = result.err().unwrap();
        println!("An error occurs while was checking to path existence!\n Error Details : {err}");
        return false;
    }
    if result.is_ok() {
        let bo: bool = result.ok().unwrap();
        if bo == false {
            println!("The path isn't exist!");
        }
        return bo;
    };
    return false;
}
pub fn create_file(path: &str, name: &str) {
    let full_path = format!("{path}/{name}");
    let last_path: &str = &full_path;

    let result: Result<File, Error> = OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .open(last_path);

    if result.is_err() {
        let err = result.err().unwrap();
        println!("Error details : {err}");
    }
}

pub fn create_file_with_path(path: &str, name: &str) {
    let is_path_exist: bool = check_path(path);

    if !is_path_exist {
        create_dir_path(path);
    }
    create_file(path, name);
}
pub fn check_file_path_and_return_full_path(path: &str, name: &str) -> String {
    
    let full_path = format!("{path}/{name}");

    let is_file_exist: bool = check_path(&full_path);

    if !is_file_exist {
        println!("File path isn't exist !");
        return String::new();
    }
    full_path
}
pub fn update_file_content(path: &str, name: &str, content: &str) {
    let full_path = check_file_path_and_return_full_path(path, name);
    if full_path.is_empty() {
        return;
    }
    let mut result: Result<File, Error> = OpenOptions::new().write(true).open(&full_path);
    let _mut_result: Result<&mut File, &mut Error> = result.as_mut();

    match _mut_result {
        Ok(file) => {
            let res: Result<(), Error> = file.write_all(content.as_bytes());
            if res.is_err() {
                let err = res.err().unwrap();
                println!("An error occured when writing to file \n Error Details : {err}");
                return;
            }
        }
        Err(err) => {
            println!("An error occured when open to file \n Error Details : {err}");
            return;
        }
    }
}
pub fn remove_dir_if_empty(path: &str) {
    let is_exist_path: bool = check_path(path);

    if !is_exist_path {
        return;
    }

    let result = std::fs::remove_dir(path);
    if result.is_err() {
        let err: std::io::Error = result.err().unwrap();
        println!("Operation has been failed with error : {err} ");
        return;
    }
}
pub fn remove_dir_all(path: &str) {
    let is_exist_path: bool = check_path(path);

    if !is_exist_path {
        return;
    }

    let result: Result<(), Error> = std::fs::remove_dir_all(path);
    if result.is_err() {
        let err: std::io::Error = result.err().unwrap();
        println!("Operation has been failed with error : {err} ");
        return;
    }
}
pub fn remove_file(path: &str, name: &str) {
    let full_path = check_file_path_and_return_full_path(path, name);
    if full_path.is_empty() {
        return;
    }
    let result: Result<(), Error> = std::fs::remove_file(full_path);
    if result.is_err() {
        let err: Error = result.err().unwrap();
        println!("An error occured while removing file! \n Error Details : {err}");
        return;
    }
}
