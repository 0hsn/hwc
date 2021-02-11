use clap::ArgMatches;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::fmt;

pub struct ArgDefinition {
    pub byte: (char, &'static str, &'static str),
    pub char: (char, &'static str, &'static str),
    pub line: (char, &'static str, &'static str),
    pub word: (char, &'static str, &'static str),
    pub file: (char, &'static str, &'static str),
}

impl Default for ArgDefinition {
    fn default() -> Self {
        Self {
            byte: ('b', "byte", "Prints number of bytes"),
            char: ('c', "char", "Prints number of characters"),
            line: ('l', "line", "Prints number of lines"),
            word: ('w', "word", "Prints number of words"),
            file: ('_', "FILE", "Filename to check from"),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
enum CounterType {
    Byte,
    Char,
    Line,
    Word,
}

pub struct CounterResult {
    values: HashMap<CounterType, u32>
}

impl fmt::Display for CounterResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_str = String::new();

        for (counter_type, count) in &self.values {
            if !result_str.is_empty() {
                result_str.push(',');
            }

            match counter_type {
                CounterType::Byte => result_str.push_str("b:"),
                CounterType::Char => result_str.push_str("c:"),
                CounterType::Line => result_str.push_str("l:"),
                CounterType::Word => result_str.push_str("w:"),
            }

            result_str.push_str(count.to_string().as_str());
        }

        write!(f, "{}", result_str)
    }
}

pub struct CounterOps {
    types: Vec<CounterType>,
    filepath: String,
}

impl CounterOps {
    pub fn from_clap_arg_matches(matches: &ArgMatches, arg_def: &ArgDefinition) -> Self {
        let mut types: Vec<CounterType> = Vec::new();
        let filepath = matches.value_of("FILE").unwrap().into();

        if matches.is_present(arg_def.byte.1) {
            types.push(CounterType::Byte);
        }

        if matches.is_present(arg_def.char.1) {
            types.push(CounterType::Char);
        }

        if matches.is_present(arg_def.line.1) {
            types.push(CounterType::Line);
        }

        if matches.is_present(arg_def.word.1) {
            types.push(CounterType::Word);
        }

        if types.is_empty() {
            types = vec![
                CounterType::Byte,
                CounterType::Char,
                CounterType::Line,
                CounterType::Word
            ];
        }

        Self {
            types,
            filepath,
        }
    }

    pub fn calculate(&self) -> CounterResult {
        let mut file = File::open(self.filepath.as_str()).unwrap();
        let mut values =  HashMap::new();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let get_byte_count = || {
            buffer.as_bytes().len() as u32
        };

        let get_char_count = || {
            buffer.trim().chars().count() as u32
        };

        let get_line_count = || {
            buffer.lines().count() as u32
        };

        let get_word_count = || {
            words_count::count(&buffer).words as u32
        };

        for item in &self.types {
            match item {
                CounterType::Byte => values.insert(
                    CounterType::Byte, get_byte_count()
                ),
                CounterType::Char => values.insert(
                    CounterType::Char, get_char_count()
                ),
                CounterType::Line => values.insert(
                    CounterType::Line, get_line_count()
                ),
                CounterType::Word => values.insert(
                    CounterType::Word, get_word_count()
                ),
            };
        }

        CounterResult {
            values
        }
    }
}
