use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

struct FileParser {
    path: PathBuf

}


impl FileParser{
    fn build(path: PathBuf) -> Result<FileParser,io::Error>{
        if(!path.exists()){
            return Err(io::Error::new(io::ErrorKind::Unsupported,"File not found!"));
        }

        
        return Ok(FileParser { path });
    }
}


fn main(){

}