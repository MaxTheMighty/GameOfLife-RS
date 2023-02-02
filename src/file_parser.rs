
use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;
use std::path::PathBuf;
use std::fmt::Error;

use crate::game_of_life::GameOfLife;


enum CHAR_MAPPING {
    ALIVE,
    DEAD,
    NEXT_LINE,
    INVALID

}

pub struct FileParser {
    file: Option<File>

}



impl FileParser{
    /* 
    pub fn build(path: PathBuf) -> Result<FileParser,io::Error>{
        if(!path.exists()){
            return Err(io::Error::new(io::ErrorKind::Unsupported,"File not found!"));
        }
        //extract this expect into a file open error
        return Ok(FileParser {file: File::open(path).expect("File open error")});
    }

    */

    pub fn create_empty() -> FileParser{
        return FileParser { file: None }

    }

    //set file and create file are similar I might want to reorganize this
    pub fn set_file(&mut self, path: PathBuf) -> Result<(),&'static str>{
        if(!path.exists()){
           return self.create_file(path);
        }
        let file_open_attempt = File::open(path);
        if(file_open_attempt.is_err()){
            return Err("Error opening existing file");
        }
        self.file = Some(file_open_attempt.unwrap());
        return Ok(());
    }

    fn create_file(&mut self,path: PathBuf) -> Result<(),&'static str>{
        let file_create_result = File::create(path);
        if(file_create_result.is_err()){
            return Err("Error creating file");
        }
        self.file = Some(file_create_result.unwrap());
        return Ok(());
    }





    
    pub fn fill_grid(&mut self, board: &mut GameOfLife) -> Result<(),&'static str>{
        if(!self.file.is_some()){
            return Err("File does not exist");
        }

        let file_clone= self.file.as_ref().unwrap();
        let reader = BufReader::new(file_clone);
        for (row_index,row) in reader.lines().enumerate(){
            if(row.is_err()){
                return Err("Error iterating over file");
            }
            let row = row.unwrap();
            for (col_index,char) in row.chars().enumerate(){
                let mut x_index = col_index;
                let mut y_index = row_index;
                let mut state = false;
                match char  {
                    'X' => {state = true},
                    'O' => {state = false},
                    '\n' => {y_index*=board.bounds},
                    _ => return Err("Invalid character in file")
                }
                board.set_cell(x_index, y_index, state);


            }
        }
        return Ok(());
    }








    

}


fn main(){

}