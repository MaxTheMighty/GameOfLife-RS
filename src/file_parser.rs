use crate::game_of_life::GameOfLife;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead,BufReader, Error, Write};
use std::path::PathBuf;

enum CHAR_MAPPING {
    ALIVE,
    DEAD,
    NEXT_LINE,
    INVALID,
}

pub struct FileParser {
    file: Option<File>,
}

impl FileParser {
    /*
    pub fn build(path: PathBuf) -> Result<FileParser,io::Error>{
        if(!path.exists()){
            return Err(io::Error::new(io::ErrorKind::Unsupported,"File not found!"));
        }
        //extract this expect into a file open error
        return Ok(FileParser {file: File::open(path).expect("File open error")});
    }

    */

    pub fn new_empty() -> FileParser {
        return FileParser { file: None };
    }

    pub fn new(path_in: PathBuf) -> Result<FileParser, io::Error> {
        let mut new_parser: FileParser = Self::new_empty();
        new_parser.set_file(path_in)?;
        return Ok(new_parser);
    }

    //set file and create file are similar I might want to reorganize this
    pub fn set_file(&mut self, path: PathBuf) -> Result<(), io::Error> {
        if (!path.exists()) {
            return self.create_file(path);
        }
        let file_open_attempt = OpenOptions::new().write(true).read(true).open(path)?;
        self.file = Some(file_open_attempt);
        return Ok(());
    }

    fn create_file(&mut self, path: PathBuf) -> Result<(), io::Error> {
        let file_create_result = File::create(path)?;
        self.file = Some(file_create_result);
        return Ok(());
    }

    pub fn fill_grid(&mut self, board: &mut GameOfLife) -> Result<(), io::Error> {
        if (!self.file.is_some()) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Attempting to fill grid from invalid file",
            ));
        }

        let file_clone = self.file.as_ref().unwrap();
        let reader = BufReader::new(file_clone);
        for (row_index, row) in reader.lines().enumerate() {
            let row = row?;
            for (col_index, char) in row.chars().enumerate() {
                let x_index = col_index;
                let mut y_index = row_index;
                let mut state = false;
                match char {
                    'X' => state = true,
                    'O' => state = false,
                    '\n' => y_index *= board.bounds,
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid character in file",
                        ))
                    } //perhaps we can just skip the character
                }
                board.set_cell(x_index, y_index, state);
            }
        }
        return Ok(());
    }

    pub fn encode_file(&mut self, board: &mut GameOfLife) -> Result<(), Error> {
        if (!self.file.is_some()) {
            self.create_file(PathBuf::from("default_save.txt"))
                .expect("Error creating default file");
        }
        let mut file_clone = self.file.as_ref().unwrap();
        let mut write_buffer: String = String::new();
        write_buffer.reserve(board.bounds * board.bounds);

        for y_index in 0..board.bounds {
            for x_index in 0..board.bounds {
                let state: bool = board.is_alive(x_index, y_index);
                write_buffer.push(self.encode_state(state));
            }
            write_buffer.push('\n');
        }
        println!("{:?}", write_buffer);
        file_clone.write_all(write_buffer.as_bytes())?;
        return Ok(());
    }

    fn encode_state(&self, state: bool) -> char {
        match state {
            true => return 'X',
            false => return 'O',
            _ => return 'O',
        }
    }
}

fn _main() {}
