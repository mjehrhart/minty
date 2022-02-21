#[allow(unused)]
pub mod meta {

    use std::cmp::Ordering;
    use std::fmt::{self, Debug };

    use crate::enums::enums::FileAction;
    use crate::enums::enums::FileType;

    #[derive(Debug, Clone, Eq,)] 
    pub struct Meta  {
        pub checksum: String,                   //done 
        pub name: String,                       //done
        pub path: String,                       //done
        pub status: FileAction,                 //default
        pub ui_event_status: bool,              //default
        pub file_points: u64,                   //done
        pub file_size: u64,                     //done 
        pub file_date: String,                  //done
        pub file_type: FileType,                //done
    }

    impl Ord for Meta {
        fn cmp(&self, other: &Self) -> Ordering {
            self.file_points.cmp(&other.file_points)
        }
    }

    impl PartialOrd for Meta {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl PartialEq for Meta {
        fn eq(&self, other: &Self) -> bool {
            self.file_size == other.file_size 
        }
    }
 
    impl Meta {
        pub fn new() -> Meta{
            Self {
                checksum: "".to_string(), 
                name: String::from(""),
                path: String::from(""),
                ui_event_status: false,
                status: FileAction::None,
                file_points: 0,
                file_size: 0,  
                file_date: String::from(""),
                file_type: FileType::None,
            }
        } 
        pub fn update(&mut self, checksum: String, name: String, path: String, file_points: u64, file_size: u64, file_date: String, file_type: FileType) -> Meta {
            Self {
                checksum,
                name,
                path,
                status: self.status,
                ui_event_status: self.ui_event_status,
                file_points,
                file_size,
                file_date,
                file_type,
            }
        }
        pub fn set_checksum(&mut self, checksum: String ) -> Meta {
            Self {
                checksum,
                name: self.name.to_string(),
                path: self.path.to_string(),
                status: self.status,
                ui_event_status: self.ui_event_status,
                file_points: self.file_points,
                file_size: self.file_size,
                file_date: self.file_date.to_string(),
                file_type: self.file_type,
            }
        } 
    }
}
