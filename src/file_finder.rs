use std::fs::File;
use std::io::Read;
use std::env::current_dir;
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
        let mut current_dir = current_dir().unwrap();
        let mut file_path = current_dir.join(FILE_NAME);
        while !file_path.exists() && current_dir.pop() {
            println!("Did not find in {:?}, looking in directory {:?}", file_path, current_dir);
            file_path = current_dir.join(FILE_NAME);
        }
        if !file_path.exists(){
            panic!("Could not find any project file named {} in any parent directories", FILE_NAME);
        }
        let path = current_dir.to_str().unwrap().to_owned();
        let f = File::open(file_path).expect("file not found");
        let content = CTFile::read(&f);
        CTFile { content, path , ctfile: f}
    }

    fn read(mut ctfile: &File) -> String{
        let mut contents = String::new();
        ctfile.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        contents
        //self.line = contents.clone().lines()
    }
}