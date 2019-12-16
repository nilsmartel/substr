use std::env::args;
use std::io::Read;

fn main() {
    let mut arg_start = 1;

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);

    let arguments: Vec<String> = args().collect();
}

struct Cut {
    lines: bool,
    start: Option<usize>,
    end: Option<usize>,
}

impl Cut {
    fn new() -> Cut {
        Cut {
            lines: false,
            start: None,
            end: None,
        }
    }

    fn from_args(self, args: &[String]) -> Cut {
        #![feature(slice_patterns, advanced_slice_patterns)]
        match args {
            [] => self,
            ["-s", index, rest..] => self.with_start(index.parse().expect("expect number after -s flag")).from_args(rest)
            ["-e", index, rest..] => self.with_end(index.parse().expect("expect number after -e flag")).from_args(rest)
        }
    }

    fn with_lines(self) -> Cut {
        self.lines = true;
        self
    }

    fn with_start(mut self, start: usize) -> Cut {
        self.start = Some(start);
        self
    }

    fn with_end(mut self, end: usize) -> Cut {
        self.end = Some(end);
        self
    }
}
