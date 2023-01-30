use std::env;

use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;
use std::path::PathBuf;



pub struct FileParser {
    file: File

}


impl FileParser{
    pub fn build(path: PathBuf) -> Result<FileParser,io::Error>{
        if(!path.exists()){
            return Err(io::Error::new(io::ErrorKind::Unsupported,"File not found!"));
        }
        return Ok(FileParser {file: File::open(path).expect("File open error")});
    }

    
    pub fn fill_grid(&self){
        let file_clone: File = self.file.try_clone().expect("File cloned");
        let reader = BufReader::new(file_clone);
        for row in reader.lines(){
            let row = row.expect("Error reading line");
            for char in row.chars(){
                println!("{}",char);
            }
        }
    }




    

}


fn main(){

}