// use calamine::{open_workbook, Reader, Xlsx};
// use rusqlite::{params, Connection, Result};
use serde::{ Serialize, Deserialize };
use std::io;
use calamine::{open_workbook, Reader, Xlsx};
use rusqlite::{params, Connection, Result, NO_PARAMS};
use rusqlite::types::Value::Null;
use walkdir::WalkDir;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::env;
use directories::UserDirs;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Debug, Serialize, Deserialize)]
struct MyMessage {
    field_str: String,
    field_u32: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Directories {
    input_directory: String,
    output_directory: String,
}


fn main() {
  tauri::Builder::default()
  //.plugin(tauri_plugin_sql::Builder::default().build())
  .invoke_handler(tauri::generate_handler![
      simple_command,
      greet,
      command_with_error,
      command_with_object,
      execute,
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

#[tauri::command]
fn simple_command() {
    println!("I was invoked from JS!");
}

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn command_with_error(arg: u32) -> Result<String, String> {
    if arg % 2 == 0 {
        Ok(format!("even value {}", arg))
    } else {
        Err(format!("odd value {}", arg))
    }
}

#[tauri::command]
fn command_with_object(message: MyMessage) -> MyMessage {
    let MyMessage {
        field_str,
        field_u32,
    } = message;

    MyMessage {
        field_str: format!("hello {}", field_str),
        field_u32: field_u32 + 11,
    }
}

#[derive(Debug)]
struct JancodeMap {
    jancode: String,
    search_str: String,
}

#[tauri::command]
//fn execute(directories: Directories) -> Result<String, String> {
fn execute(directories: Directories) -> Result<String, String> {
    
    // let Directories {
    //     input_directory,
    //     output_directory,
    // } = directories;

    println!("execute function");

    // let directories = Directories { 
    //     input_directory: String::from("input"),
    //     output_directory: String::from("input") };

    let in_directory = &directories.input_directory; // clone input_directory
    println!("{}", in_directory); 
    let out_directory = &directories.output_directory; // clone input_directory
    let out_directory2 = &directories.output_directory; // clone input_directory
    println!("{}", out_directory); 

    //do_proc(in_directory, out_directory);
    match do_proc(in_directory, out_directory) {
        Ok(_) => { 
            println!("do_proc executed successfully");
            let db_path = format!("{}/{}", out_directory2, "my_database.db3");
            match fs::remove_file(&db_path) {
                Ok(_) => println!("Successfully deleted database file"),
                Err(e) => println!("Failed to delete database file: {}", e),
            }
                
        },
        Err(e) => println!("do_proc failed with error: {}", e),
    }

    Ok(format!("{}", "Hello"))

}


fn do_proc(in_directory: &String, out_directory: &String) -> Result<()> {
//fn do_proc(in_directory: String, out_directory: String) -> Result<(), rusqlite::Error, std::io::Result<()> {
    
    println!("do_proc 1");

    //let user_dirs = UserDirs::new().ok_or("Couldn't get user directories")?;
    // let user_dirs = UserDirs::new().ok_or_else(|| rusqlite::Error::new(rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(rusqlite::ffi::ErrorCode::IoError), Some("Couldn't get user directories".to_string()))))?;

    // let mut db_path: PathBuf = user_dirs.home_dir().into();
    // db_path.push("my_database.db3");

    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Couldn't get the current directory: {}", e),
    };
     println!("The current directory is {}", current_dir.display());
    
    // let conn = Connection::open("./my_database.db3")?;
    
    let conn = match Connection::open(out_directory.clone()+ "/my_database.db3") {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error opening database: {}", e);
            return Err(e);
        }
    };

    println!("do_proc 2");

    // excel読込み
    let path = in_directory.clone() + "/Sample_ItemInfo.xlsx";
    conn.execute("DROP TABLE IF EXISTS map",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS map (
            jancode              TEXT PRIMARY KEY,
            search_str            TEXT NOT NULL
                  )",
        params![],
    )?;

    println!("do_proc 3");

    let mut excel: Xlsx<_> = open_workbook(path).unwrap();

    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {

        for row in r.rows() .skip(1) {
            println!("{:?}", row);

            // let jancode = row[0].get_string().unwrap();
            // let search_str = row[1].get_string().unwrap();
            // let jancode = row[0].get_string();
            // let search_str = row[1].get_string();

            // println!("{}", jancode);
            // println!("{}", search_str);

            let jancode = &row[0];
            let search_str = &row[1];
            //let int_value: i32 = search_str as i32;

            // convert to string
            match jancode {
                calamine::DataType::String(s) => println!("String: {}", s),
                calamine::DataType::Float(f) => { 
                    println!("Float: {}", f); 
                    let converted: i64 = *f as i64; 
                    let jancode = converted.to_string(); 
                    //println!("search_str: {}", search_str);
                },
                calamine::DataType::Int(i) => println!("Integer: {}", i),
                _ => println!("Other type"),
            }

            // convert to string
            match search_str {
                calamine::DataType::String(s) => println!("String: {}", s),
                calamine::DataType::Float(f) => { 
                    println!("Float: {}", f); 
                    let converted: i64 = *f as i64; 
                    let search_str = converted.to_string(); 
                    //println!("search_str: {}", search_str);
                },
                calamine::DataType::Int(i) => println!("Integer: {}", i),
                _ => println!("Other type"),
            }

            println!("jancode: {}", jancode);
            println!("search_str: {}", search_str);

            conn.execute(
                "INSERT INTO map (jancode, search_str) VALUES (?1, ?2)",
                params![jancode.to_string(), search_str.to_string()],
            )?;

        }

        println!("do_proc 4");

        // DB行毎のループ処理
        // let mut stmt = conn.prepare("SELECT * FROM map ORDER BY jancode ASC")?;
        // let jancode_maps = stmt.query_map(NO_PARAMS, |row| {
        //     Ok(JancodeMap {
        //         jancode: row.get(0).unwrap(),
        //         search_str: row.get(1).unwrap(),
        //     })
        // }).unwrap();

        // println!("DB行読込み");
        // for jancode_map in jancode_maps {
        //     // let row = row?;
        //     // let db_col_jancode: String = row.get(0)?;
        //     //let db_col_search_str: String = row.get(0)?;
        //     //println!("jancode: {}, search_str: {}", db_col_jancode, db_col_search_str);
        //     println!("{:?}", jancode_map.unwrap());
        // }
    
    }

    // ディレクトリ内のファイルを読み込む
    // let directory_in = "input_dir";
    let directory_in = in_directory;
    // 出力先ディレクトリ
    // let directory_out = "output_dir";
    let directory_out = out_directory;
    // replace "directory_path" with your actual directory path
    match fs::create_dir_all(directory_out) {
        Ok(_) => (),
        Err(e) => println!("Failed to create directory: {}", e),
    }

    conn.execute("DROP TABLE IF EXISTS image_path",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS image_path (
            image_name      TEXT PRIMARY KEY,
            path            TEXT NOT NULL
                  )",
        params![],
    )?;
    for entry in WalkDir::new(directory_in) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let extension = path.extension().and_then(std::ffi::OsStr::to_str);
        let filename = path.file_name().and_then(std::ffi::OsStr::to_str);
    
        match extension {
            Some("jpg") | Some("png") => {
                println!("{}", path.display());

            conn.execute(
                "INSERT INTO image_path (image_name, path) VALUES (?1, ?2)",
                params![filename, path.display().to_string()],
            )?;

            },
            _ => {},
        }
    }    


    // DB行毎のループ処理
    let mut stmt = conn.prepare("SELECT * FROM map ORDER BY jancode ASC")?;
    let jancode_maps = stmt.query_map(NO_PARAMS, |row| {
        Ok(JancodeMap {
            jancode: row.get(0).unwrap(),
            search_str: row.get(1).unwrap(),
        })
    }).unwrap();

    println!("DB行読込み");
    // マップ毎に、ファイル名を検索して、ファイルパスをコピーする
    let mut hit_target_count = 0;
    for jancode_map in jancode_maps {
        // let jancode = jancode_map.unwrap().jancode;
        // let search_str = jancode_map.unwrap().search_str;
        let jancode_map = jancode_map.unwrap();
        let jancode = jancode_map.jancode;
        let search_str = jancode_map.search_str;

        println!("{:?}", search_str);

        let like_search_image_name = format!("%{}%", search_str);
        let mut stmt = conn.prepare("SELECT * FROM image_path WHERE image_name like ?1 ORDER BY image_name ASC")?;
        let rows = stmt.query_map(params![like_search_image_name], |row| {
            let image_name: String = row.get(0).unwrap();
            let path: String = row.get(1).unwrap();

            Ok((image_name, path))
        })?;

        // jancode毎の処理件数
        let mut copy_count = 0;

        for target in rows {
            hit_target_count += 1;
            let (image_name, path) = target.unwrap();

            // 初回のみディレクトリ作成し、ファイル名も変換してコピー
            if copy_count == 0 {

                let jancode_directory = format!("{}/{}", directory_out, jancode);
                // let jancode_directory = format!("{}/{}", directory_out.clone(), jancode);
                match fs::create_dir_all(jancode_directory) {
                // match fs::create_dir_all(jancode_directory.clone()) {
                        Ok(_) => (),
                    Err(e) => println!("Failed to create directory: {}", e),
                }

                let source_path = path;
                // replace "destination_directory" with your actual destination directory
                let destination_path = format!("{}/{}/{}", directory_out, jancode, "thumbnail.jpg");
                // let destination_path = format!("{}/{}/{}", directory_out.clone(), jancode, "thumbnail.jpg");

                println!("{:?}", destination_path);

                match fs::copy(&source_path, &destination_path) {
                    Ok(_) => println!("Successfully copied file"),
                    Err(e) => println!("Failed to copy file : {}", e),
                }
    
                match fs::create_dir_all(directory_out) {
                    Ok(_) => (),
                    Err(e) => println!("Failed to create directory: {}", e),
                }
                

            } else {

                let source_path = path;
                // replace "destination_directory" with your actual destination directory
                let destination_path = format!("{}/{}/{}", directory_out, jancode, image_name);
                // let destination_path = format!("{}/{}/{}", directory_out.clone(), jancode, image_name);
                match fs::copy(&source_path, &destination_path) {
                    Ok(_) => println!("Successfully copied file"),
                    Err(e) => println!("Failed to copy file 2: {}", e),
                }


            }

            copy_count += 1;
                        
            // println!("{:?}", target.unwrap());
            
            println!("{:?}", image_name);


        }


    }

    println!("検索結果 {:?}", hit_target_count);
    
    // Delete the database file

    Ok(())
}

fn execute_origin(directories: Directories) -> Result<String, String> {

    println!("execute function");

    let in_directory = directories.input_directory.clone(); // clone input_directory
    println!("{}", in_directory); 
    let out_directory = directories.output_directory.clone(); // clone input_directory
    println!("{}", out_directory); 

    
    Ok(format!("{}", "Hello"))

}