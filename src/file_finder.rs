use std::fs::File;
use std::io::Read;
static FILE_NAME: &str = ".ctproject";


pub struct CTFile{
    pub content: String,
    pub path: String,
    pub ctfile: File
}

impl CTFile{
    pub fn get_content() -> CTFile{
        CTFile::find_ctproject()
       // ct_file.read()
    }

    fn find_ctproject() -> CTFile{
        let f = File::open(FILE_NAME).expect("file not found");
        let content = CTFile::read(&f);
        CTFile { content, path: FILE_NAME.to_string(), ctfile: f}
    }

    fn read(mut ctfile: &File) -> String{
        let mut contents = String::new();
        ctfile.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        contents
        //self.line = contents.clone().lines()
    }
}