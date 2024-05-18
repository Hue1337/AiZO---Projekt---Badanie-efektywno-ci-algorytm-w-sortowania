use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::Path;


use std::io::prelude::*;

pub fn read_data_i32(filename: String) -> Vec<i32> {
    println!("data_handlers::read_data");
    let mut data_result: Vec<i32> = Vec::new();
    // Checking if file exists
    let data_str: String = fs::read_to_string(filename).expect("Failed to read data!");

    let mut tmp_str: String = String::new();
    for el in data_str.chars() {
        if el == '\n' && tmp_str.len() > 0 {
            data_result.push(tmp_str.parse::<i32>().unwrap());
            tmp_str = String::new();
        } else {
            tmp_str.push(el);
        }
    }
    data_result.drain(0..1);

    data_result
}

pub fn read_data_f32(filename: String) -> Vec<f32> {
    println!("data_handlers::read_data");
    let mut data_result: Vec<f32> = Vec::new();
    // Checking if file exists
    let data_str: String = fs::read_to_string(filename).expect("Failed to read data!");

    let mut tmp_str: String = String::new();
    for el in data_str.chars() {
        if el == '\n' && tmp_str.len() > 0 {
            data_result.push(tmp_str.parse::<f32>().unwrap());
            tmp_str = String::new();
        } else {
            tmp_str.push(el);
        }
    }
    data_result.drain(0..1);

    data_result
}

pub fn read_data_f64(filename: String) -> Vec<f64> {
    println!("data_handlers::read_data");
    let mut data_result: Vec<f64> = Vec::new();
    // Checking if file exists
    let data_str: String = fs::read_to_string(filename).expect("Failed to read data!");

    let mut tmp_str: String = String::new();
    for el in data_str.chars() {
        if el == '\n' && tmp_str.len() > 0 {
            data_result.push(tmp_str.parse::<f64>().unwrap());
            tmp_str = String::new();
        } else {
            tmp_str.push(el);
        }
    }
    data_result.drain(0..1);

    data_result
}

pub fn write_data_i32(data: Vec<i32>, filename: String) -> std::io::Result<()> {
    file_wipeout(filename.clone());
    path_valdation();
    file_validation(filename.clone());
    let mut file_tmp = OpenOptions::new()
        .append(true)
        .write(true)
        .open(filename.clone())
        .unwrap();
    let mut str_tmp: String = String::new();

    // Zapis rozmiaru tablicy do pliku
    writeln!(file_tmp, "{}", data.len().to_string());

    // Zapis wartosci tablicy do pliku
    for i in data {
        str_tmp = i.to_string();
        if let Err(e) = writeln!(file_tmp, "{}", str_tmp) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    Ok(())
}

pub fn write_data_f32(data: Vec<f32>, filename: String) -> std::io::Result<()> {
    file_wipeout(filename.clone());
    path_valdation();
    file_validation(filename.clone());
    let mut file_tmp = OpenOptions::new()
        .append(true)
        .write(true)
        .open(filename.clone())
        .unwrap();
    let mut str_tmp: String = String::new();

    // Zapis rozmiaru tablicy do pliku
    writeln!(file_tmp, "{}", data.len().to_string());

    // Zapis wartosci tablicy do pliku
    for i in data {
        str_tmp = i.to_string();
        if let Err(e) = writeln!(file_tmp, "{}", str_tmp) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    Ok(())
}

pub fn write_data_f64(data: Vec<f64>, filename: String) -> std::io::Result<()> {
    file_wipeout(filename.clone());
    path_valdation();
    file_validation(filename.clone());
    let mut file_tmp = OpenOptions::new()
        .append(true)
        .write(true)
        .open(filename.clone())
        .unwrap();
    let mut str_tmp: String = String::new();

    // Zapis rozmiaru tablicy do pliku
    writeln!(file_tmp, "{}", data.len().to_string());

    // Zapis wartosci tablicy do pliku
    for i in data {
        str_tmp = i.to_string();
        if let Err(e) = writeln!(file_tmp, "{}", str_tmp) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    Ok(())
}



fn file_validation(filename: String){
    let tmp_file = File::open(filename.clone());
    let tmp_file = match tmp_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename.clone()) {
                Ok(fcreate) => fcreate,
                Err(err) => panic!("Failed to create a file [data.csv]: {:?}", err),
            }
            other_error => {
                panic!("Failed to open a file [data.csv]");
            }
        },
    };
}

fn path_valdation() {
    if !(Path::new("./config/").exists()){
        fs::create_dir("./config/");
    }

    if !(Path::new("./data/").exists()) {
        fs::create_dir("./data/");
    }
}

fn file_wipeout(filename: String) {
    match fs::remove_file(filename.clone()) {
        Ok(_) => println!("File successfuly wiped: {}", filename),
        Err(err) => println!("Failed to wipe the file: {}", filename),
    };
}

pub fn list_avaible_arrays() {
    println!("List of avaible data files:");
    for file in fs::read_dir("./data").unwrap() {
            let tmp_str: String = file.expect("Failed").file_name().to_str().expect("Failed to convert!").to_string();
            println!("\t{}", tmp_str);
    }
}