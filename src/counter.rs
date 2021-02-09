use clap::ArgMatches;
use std::fs::File;
use std::io::Read;
use Default;

#[derive(Debug)]
enum CounterType {
    Byte,
    Char,
    Line,
    Word,
}

#[derive(Debug)]
enum CounterResult {
    Byte(u32),
    Char(u32),
    Line(u32),
    Word(u32),
}

impl Default for CounterResult {
    fn default() -> Self {
        Self(0u32)
    }
}

#[derive(Debug)]
pub struct CounterOps {
    types: Vec<CounterType>,
    filepath: String,
}

impl CounterOps {
    pub fn from_clap_arg_matches(matches: &ArgMatches) -> Self {
        let mut types: Vec<CounterType> = Vec::new();
        let filepath = matches.value_of("FILE").unwrap().into();

        if matches.is_present("byte") {
            types.push(CounterType::Byte);
        }

        if matches.is_present("char") {
            types.push(CounterType::Char);
        }

        if matches.is_present("line") {
            types.push(CounterType::Line);
        }

        if matches.is_present("word") {
            types.push(CounterType::Word);
        }

        Self {
            types,
            filepath,
        }
    }

    pub fn display(&self) {
        self.calc();
        println!("{:#?}", self);
    }

    fn calc(&self) {
        let mut file = File::open(self.filepath.as_str()).unwrap();
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        for item in &self.types {
            match item {
                CounterType::Byte => {
                    println!("bytes: {}", self.get_byte_count(&buffer));
                },
                CounterType::Char => {
                    println!("characters: {}", self.get_char_count(&buffer));
                },
                CounterType::Line => {
                    println!("lines: {}", self.get_line_count(&buffer));
                },
                CounterType::Word => {
                    // println!("words: {}", self.get_word_count(&buffer));
                },
            }
        }
    }

    fn get_byte_count(buffer: &String) -> u32 {
        buffer.as_bytes().len() as u32
    }

    fn get_char_count(buffer: &String) -> u32 {
        buffer.trim().chars().count() as u32
    }

    fn get_line_count(buffer: &String) -> u32 {
        buffer.lines().count() as u32
    }

    fn get_word_count(buffer: &String) -> u32 {
        2
    }
}
