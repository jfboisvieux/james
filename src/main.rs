use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::path::PathBuf;
use std::string::String;
use std::io::Read;
use dirs::home_dir;
use std::collections::HashMap;


fn main() {

    let mut path: PathBuf = home_dir().unwrap();
    let  dic: HashMap<String, String>;
    path.push(".config/secret.pwd");
    let args: Vec<String> = std::env::args().collect();
    let query = &args[1].to_string();


    if query == "n" {
        store_data(path.clone()) }
    else if query == "s" {
        let key = new_key();

        dic = file_to_hash(path);
        let passwd = dic.get(&key);
        match passwd {
            Some(pwd) => println!("password : {}",pwd),
            None => println!("Identifiant inconnu !")
        }
    }
    else if query == "all" {
        for (key, value) in file_to_hash(path){
            println!("{} : {}", key, value);
        }
    }
}



fn store_data(path: PathBuf) {
    let key: String = new_key();
    let repository: Vec<String> = read_file(path.clone());
    for  data in repository {
        let  stored_key = data.split("$").next().unwrap();
        if key == stored_key {
            println!("This identifier has a stored password");
            return;
        }
    }
    let passwd = new_passwd();
    let mut new_key_passwd = key.to_string() + &"$".to_string();
    new_key_passwd = new_key_passwd +    &passwd.to_string();
    save_to_file(&path, new_key_passwd);
}

fn new_key() -> String {
    let mut key = String::new();
    println!(" enter site/computer name ");
    io::stdin()
        .read_line(&mut key)
        .expect("unable to read user input ");
    key.pop();
    key
}

fn new_passwd() -> String {
    let mut passwd = String::new();
    println!("enter password ");
    io::stdin()
        .read_line(&mut passwd)
        .expect("unable to read user input ");
    passwd
}


fn save_to_file(path: &PathBuf, new_data: String) {
    if path.exists() {
        let mut f = File::options()
            .append(true)
            .write(true)
            .read(true)
            .open(&path)
            .unwrap();
        f.write_all(new_data.as_bytes()).unwrap();
    } else {
        let f = File::create(&path);
        match f {
            Err(e) => println!("erreur  in else: {}", e),
            Ok(mut f) => f.write_all(new_data.as_bytes()).unwrap(),
        };
    }
}



fn read_file(path: PathBuf) -> Vec<String> {
    let mut output_string =  String::new();
    let output_vector_of_string: Vec<String>;
    if path.exists() {
        let f  = File::options()
            .read(true)
            .open(&path)
            .unwrap();

        let mut buffer_reader = BufReader::new(f);
        buffer_reader.read_to_string(&mut output_string).expect("unable to read ");

    }
    else {
        let _ = File::create(&path);
    }
    output_vector_of_string = output_string.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
    output_vector_of_string

}




fn file_to_hash(path: PathBuf) -> HashMap<String, String> {
    let mut passwd_dic: HashMap<String, String> = HashMap::new();
    let f = File::open(&path).expect("error");
    let   reader = BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut iter = line.split("$");
                let key = &iter.next().unwrap();
                let value = &iter.next().unwrap();
                // println!("key = {} value = {}", key, value);
                passwd_dic.insert(key.to_string(), value.to_string());
            }

            Err(e) => println!("{}", e),
        }
    }
    passwd_dic
}
