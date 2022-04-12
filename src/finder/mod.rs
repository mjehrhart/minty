// use std::{
//     path::PathBuf,
//     sync::{Arc, Mutex},
// };

#[allow(unused)]
use super::enums;

#[allow(unused, dead_code)]
pub mod finder {

    use crate::enums::enums::{self, *};
    use crate::file::meta::Meta;
    use crate::file::{self, meta};

    use std::collections::HashMap;
    use std::ffi::OsStr;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use jwalk::WalkDir;
    use rayon::prelude::*;
    use regex::Regex;
    use tokio::io::{self, AsyncReadExt};
    use tokio::task;
    use tokio::{fs, join};
    use walkdir::DirEntry;

    use std::sync::mpsc::channel;
    use std::thread;

    #[derive(Debug, Clone)]
    pub struct Finder {
        pub bucket: String,
        pub chunk_size: u64,
        pub flag_view: bool,
        pub flag_remove: bool,
        pub(crate) matching: FileAction,
        pub data_set: HashMap<String, Vec<Meta>>,
        pub starting_directory: String,
    }

    impl Finder {
        pub fn new() -> Finder {
            Finder {
                bucket: String::from("./"),
                chunk_size: 0,
                flag_view: false,
                flag_remove: false,
                matching: FileAction::None,
                data_set: HashMap::new(),
                starting_directory: String::from(""),
            }
        }
        //Directory Walker
        pub async fn rayon_walk_dir(&mut self, path: &str, filter: [bool; 5]) {
            //*************************************************************************************************************************//
            fn read_dir(
                entries: Arc<Mutex<Vec<(String, String, String, String, u64, u64)>>>,
                s: &rayon::Scope<'_>,
                base_path: PathBuf,
                chunk_size: u64,
                filter: [bool; 5],
            ) {
                //Works Belows
                let bp = base_path.clone();
                let temp = base_path.file_name().unwrap();
                let path: String = String::from(temp.to_string_lossy());

                let flag = !path.starts_with('.');
                if flag {
                    // for entry in std::fs::read_dir(&bp).unwrap_or_else(|e| {
                    //     panic!("Error reading dir: {:?}, {}", temp, e);
                    //     //process::exit(1);
                    // })
                    // {

                    // }

                    for entry in std::fs::read_dir(bp).unwrap_or_else(|e| {
                        panic!("Error reading dir: {:?}, {}", temp, e);
                        //process::exit(1);
                    }) {
                        let entry = entry;

                        match &entry {
                            Ok(ent) => {
                                let entry = entry.unwrap();
                                let path = entry.path();

                                if !path.starts_with(".") {
                                    let metadata = entry.metadata().unwrap();

                                    if metadata.is_dir() {
                                        let move_entries = entries.clone();
                                        s.spawn(move |s1| {
                                            read_dir(move_entries, s1, path, chunk_size, filter)
                                        });
                                    } else if metadata.is_file() {
                                        let p = path.as_path().display().to_string();

                                        let ft = Finder::get_file_type(&p);
                                        let mut flag_continue = false;
                                        match ft {
                                            FileType::Audio => {
                                                if filter[0] {
                                                    flag_continue = true;
                                                }
                                            }
                                            FileType::Document => {
                                                if filter[1] {
                                                    flag_continue = true;
                                                }
                                            }
                                            FileType::Image => {
                                                if filter[2] {
                                                    flag_continue = true;
                                                }
                                            }
                                            FileType::Other => {
                                                if filter[3] {
                                                    flag_continue = true;
                                                }
                                            }
                                            FileType::Video => {
                                                if filter[4] {
                                                    flag_continue = true;
                                                }
                                            }
                                            FileType::None => {}
                                            FileType::All => {
                                                flag_continue = true;
                                            }
                                        }

                                        if flag_continue == true {
                                            let async_results =
                                                Finder::async_file_metadata_join(&p, chunk_size);
                                            let x = futures::executor::block_on({ async_results });

                                            entries.lock().unwrap().push(x);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("{:?}", &entry);
                                println!("####################{}", e);
                            }
                        }
                    }
                } else {
                    //do nothing
                }
            }

            //*************************************************************************************************************************//
            pub fn walk_files(
                base_path: &std::path::Path,
                chunk_size: u64,
                filter: [bool; 5],
            ) -> std::vec::Vec<(String, String, String, String, u64, u64)> {
                let entries = Arc::new(Mutex::new(Vec::new()));

                let base_path = base_path.to_owned();
                let move_entries = entries.clone();
                let ret = rayon::scope(move |s| {
                    s.spawn(move |s1| read_dir(move_entries, s1, base_path, chunk_size, filter))
                });

                let entries = Arc::try_unwrap(entries).unwrap().into_inner().unwrap();
                entries
            }
            //*************************************************************************************************************************//

            let path = std::path::Path::new(path);

            let flag = !path.starts_with(".");

            if true {
                let mut x = walk_files(path, self.chunk_size, filter);
                for item in x {
                    let metadata = std::fs::metadata(&item.1); //unwrap()
                    match metadata {
                        Ok(md) => {
                            if !md.is_dir() {
                                let path: String = item.1.clone();

                                let ft = Finder::get_file_type(&path);

                                let mut flag_continue = false;
                                match ft {
                                    enums::FileType::Image => {
                                        flag_continue = true;
                                    }
                                    FileType::Audio => {
                                        flag_continue = true;
                                    }
                                    FileType::Video => {
                                        flag_continue = true;
                                    }
                                    FileType::Document => {
                                        flag_continue = true;
                                    }
                                    FileType::Other => {
                                        flag_continue = true;
                                    }
                                    FileType::None => {}
                                    FileType::All => {
                                        flag_continue = true;
                                    }
                                }

                                //Continue if FileType equals "...."
                                if flag_continue == true {
                                    let mut meta = meta::Meta {
                                        checksum: item.2.clone(),
                                        name: item.0,
                                        path: item.1,
                                        status: FileAction::None,
                                        ui_event_status: false,
                                        file_points: item.4,
                                        file_size: item.5,
                                        file_date: item.3,
                                        file_type: ft,
                                    };

                                    self.insert_item(item.2, meta);
                                }
                            }
                        }
                        Err(_) => {
                            println!("Some big error here. but does the program exit???")
                        }
                    }
                }
            }
        }
        pub async fn slow_walk_dir(&mut self, path: &str) {
            fn is_hidden(entry: &walkdir::DirEntry) -> bool {
                //.map(|s| s.starts_with("."))
                entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with('.'))
                    .unwrap_or(false)
            }

            //let me = self;
            walkdir::WalkDir::new(path)
                .into_iter()
                .par_bridge()
                //.filter(|entry| { /* filter if you like */ })
                .for_each(|ent| {
                    let entry = ent.unwrap().clone();
                    if entry.path().is_dir() {
                        // do nothing if file is a directory
                    } else {
                        let name = entry
                            .path()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned();
                        let path = entry.path().to_string_lossy().into_owned();

                        //println!("--->{:?}", &entry);

                        let ft = Finder::get_file_type(&path);

                        let mut flag_continue = false;
                        match ft {
                            enums::FileType::Image => {
                                flag_continue = true;
                            }
                            FileType::Audio => {
                                flag_continue = true;
                            }
                            FileType::Video => {
                                flag_continue = true;
                            }
                            FileType::Document => {}
                            FileType::Other => {}
                            FileType::None => {}
                            FileType::All => {
                                flag_continue = true;
                            }
                        }

                        //Continue if FileType equals "...."
                        if flag_continue == true {
                            let async_results =
                                Finder::async_file_metadata_join(&path, self.chunk_size);
                            // let (fnc, fc, fsp, fs, ft) =  x.await;
                            let x = futures::executor::block_on({ async_results });
                            println!("--->{:?}", &x);
                            let mut meta = meta::Meta {
                                checksum: x.2.clone(),
                                name: x.0,
                                path: x.1,
                                status: FileAction::None,
                                ui_event_status: false,
                                file_points: x.4,
                                file_size: x.5,
                                file_date: x.3,
                                file_type: FileType::None,
                            };
                        }
                    }
                });
        }
        pub async fn fast_walk_dir(&mut self, path: &str) {
            for dir_entry_result in jwalk::WalkDirGeneric::<((), Option<u64>)>::new(&path)
                .skip_hidden(true)
                .parallelism(jwalk::Parallelism::RayonNewPool(20))
                .process_read_dir(|_, dir_entry_results| {})
            {
                match dir_entry_result {
                    Ok(dir_entry) => {
                        //// Sample Return Tuple
                        //// (("bab96be13d9817317006e73bae07987c", "01/06/2018", 0, 446984, "image"),)

                        if !dir_entry.file_type.is_dir() {
                            let path: String = dir_entry.path().as_path().display().to_string();

                            let ft = Finder::get_file_type(&path);

                            let mut flag_continue = false;
                            match ft {
                                enums::FileType::Image => {
                                    flag_continue = true;
                                }
                                FileType::Audio => {
                                    flag_continue = true;
                                }
                                FileType::Video => {
                                    flag_continue = true;
                                }
                                FileType::Document => {}
                                FileType::Other => {}
                                FileType::None => {}
                                FileType::All => {
                                    flag_continue = true;
                                }
                            }

                            //Continue if FileType equals "...."
                            if flag_continue == true {
                                let async_results =
                                    Finder::async_file_metadata_join(&path, self.chunk_size);
                                // let (fnc, fc, fsp, fs, ft) =  x.await;
                                let x = tokio::join!(async_results);

                                let mut meta = meta::Meta {
                                    checksum: x.0 .2.clone(),
                                    name: x.0 .0,
                                    path: x.0 .1,
                                    status: FileAction::None,
                                    ui_event_status: false,
                                    file_points: x.0 .4,
                                    file_size: x.0 .5,
                                    file_date: x.0 .3,
                                    file_type: FileType::None,
                                };

                                self.insert_item(x.0 .2, meta);
                            }
                        }
                    }
                    Err(error) => {
                        println!("Read dir_entry error: {}", error);
                        //return "Error".to_string()
                    }
                }
            }
        }
        //ActionEvents
        fn insert_item(&mut self, checksum: String, mut meta: Meta) {
            if !self.data_set.contains_key(&checksum) {
                let mut vector: Vec<Meta> = vec![];
                meta.status = FileAction::Read;
                vector.push(meta);
                self.data_set.insert(checksum, vector);
            } else {
                if let Some(v) = self.data_set.get_mut(&checksum) {
                    meta.ui_event_status = true;
                    meta.status = FileAction::Delete;
                    v.push(meta);
                }
            }
        }
        //User ActionEvents
        pub fn adjust_file_order(&mut self) -> Self {
            for item in self.data_set.iter_mut() {
                let (_, mut v) = item;

                v.sort_by(|a, b| {
                    //println!("{:?}", &a.status);
                    a.file_points.cmp(&b.file_points)
                });

                let mut flag = true;
                for mut row in v {
                    if flag == true {
                        row.status = FileAction::Save;
                        row.ui_event_status = false;
                    } else {
                        row.status = FileAction::Delete;
                        row.ui_event_status = true;
                    }
                    flag = false;
                }
            }

            Self {
                bucket: self.bucket.to_string(),
                chunk_size: self.chunk_size,
                flag_view: self.flag_view,
                flag_remove: self.flag_remove,
                matching: self.matching,
                data_set: self.data_set.clone(),
                starting_directory: self.starting_directory.to_string(),
            }
        }
        //File Meta
        async fn async_file_metadata_join(
            path: &str,
            chunk_size: u64,
        ) -> (String, String, String, String, u64, u64) {
            //// -1
            let fno = Finder::get_file_name_os(path);

            //// 0
            let fpo = Finder::get_file_path_os(path);

            //// 1
            let fnc = Finder::get_file_byte_checksum(path, chunk_size);

            //// 2
            let fc = Finder::get_file_created(path);

            //// 3
            let fps = Finder::get_file_points_system(path);

            //// 4
            let fs = Finder::get_file_size(path);

            join!(fno, fpo, fnc, fc, fps, fs)
        }
        async fn get_file_byte_checksum(path: &str, chunk_size: u64) -> String {
            //println!("{}", path);
            let mut contents: Vec<u8> = Vec::new();
            let ft = Finder::get_file_type(&path);
            let mut chunk_size = chunk_size;
            match ft {
                enums::FileType::Image => {
                    chunk_size = 57344; //57.344kb
                }
                FileType::Audio => {
                    chunk_size = 8192; //8.192kp
                }
                FileType::Video => {
                    chunk_size = 28672; //28.672kb
                }
                FileType::Document => {
                    chunk_size = 163840; //163.84kb  needs to be high for PDF matching
                }
                FileType::Other => {
                    chunk_size = 8192; //8.192kp
                }
                FileType::None => {
                    chunk_size = 0;
                }
                FileType::All => {
                    chunk_size = 0;
                }
            }

            let f = std::fs::File::open(path);
            match f {
                Ok(file) => {
                    //Read first x number of bytes
                    let mut chunk = std::io::Read::take(file, chunk_size);
                    let _byte_count = std::io::Read::read_to_end(&mut chunk, &mut contents)
                        .expect("Unable to read");

                    //Checksum
                    let digest = md5::compute(contents);
                    let checksum = format!("{:x}", digest);

                    return checksum;
                }
                Err(e) => return format!("{}", e),
            }
        }
        async fn get_file_name_os(path: &str) -> String {
            let name = std::path::Path::new(&path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            name.to_string()
        }
        async fn get_file_path_os(path: &str) -> String {
            path.to_string()
        }
        async fn get_file_points_system(path: &str) -> u64 {
            let re = Regex::new(r"(?i)copy|duplicate|dup|bu|backup|safe|saved").unwrap();
            let c = re.find_iter(&path).count();
            c.try_into().unwrap()
        }
        async fn get_file_size(path: &str) -> u64 {
            let file_size = match std::fs::metadata(&path) {
                Ok(fs) => {
                    let s = fs.len();
                    let size = match TryInto::<i64>::try_into(s) {
                        Ok(f) => return f.try_into().unwrap(),
                        Err(e) => {
                            println!("ERROR::{}, {}", &path, e);
                            return 0;
                        }
                    };
                }
                Err(_) => return 0,
            };
        }
        async fn get_file_created(path: &str) -> String {
            let metadata = match std::fs::metadata(path) {
                Ok(md) => {
                    if let Ok(time) = md.modified() {
                        let datetime: chrono::DateTime<chrono::Local> = time.into();
                        let t: String = datetime.format("%m/%d/%Y").to_string();
                        //println!("get_created_date::{}", t);
                        return t;
                    } else {
                        println!("1: Not supported on this platform or filesystem: {}", path);
                    }
                }
                Err(_) => return format!("Not supported on this platform or filesystem: {}", path),
            };

            String::from("Error:1001")
        }
        fn get_file_type(path: &str) -> crate::enums::enums::FileType {
            let ext = std::path::Path::new(&path)
                .extension()
                .and_then(OsStr::to_str);

            match ext {
                None => return FileType::None,
                Some(_) => {
                    let ext = ext.unwrap().to_lowercase();
                    match ext.as_str() {
                        "jpg" | "png" | "heic" | "jpeg" | "tiff" | "tif" | "psd" | "tga"
                        | "thm" | "dds" => return FileType::Image,
                        "avi" | "mov" | "mpg" | "mpeg" | "mp4" => return FileType::Video,
                        "doc" | "docx" | "txt" | "vcs" | "xls" | "pdf" | "ppt" | "zip" => {
                            return FileType::Document
                        }
                        "tta" | "sln" | "mogg" | "oga" | "wma" | "wav" | "vox" | "voc" | "raw"
                        | "ogg" | "mpc" | "mp3" | "m4p" | "m4b" | "m4a" | "gsm" | "flac" | "au"
                        | "ape" | "amr" | "aiff" | "act" | "aax" | "aac" | "aa" | "3gp" => {
                            return FileType::Audio
                        }
                        _ => return FileType::Other,
                    };
                }
            }

            //println!("get_file_type)_ ::{}", &path);
            let ext = ext.unwrap().to_lowercase();
        }
    }

    //Helpers
    pub fn type_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }
}
