
#[allow(unused)]
pub mod enums {

    #[derive(Debug, PartialEq, Copy, Clone, Eq)]
    pub enum FileAction {
        Delete,
        Save,
        None,
        Read, //got checksum
    }

    #[derive(Debug, PartialEq, Copy, Clone, Eq)]
    pub enum FileType {
        Image,
        Audio,
        Video,
        Document,
        Other,
        None, 
        All
    }

    #[derive(Debug, PartialEq, Clone, Copy, Eq)]
    pub enum MetaData<'a > {
        FileSize(i32),
        Created(&'a str), 
    }

    
}

 