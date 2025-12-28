use std::fs::File;
use std::io::{self, BufRead, BufReader};

trait DiagnosticCode where Self: std::fmt::Debug {

fn message(&self) -> &'static str;
    fn code_str(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorCode {
    ET001,
    ET002,
    ET003,
    ET004,
    ET005,
    ET006,
    ET007,
    ET008,
    ET009,
    ET010,
    ET011,
    ET012,
    EP001,
    EP002,
    EP003,
    EP004,
    EP005,
    EP006,
    EP007,
    EP008,
    EP009,
    EP010,
    EP011,
    EP012,
    EP013,
    EP014,
    EP015,
    EP016,
    EP017,
    EP018,
    EP019,
    EP020,
    EP021,
    EP022,
}

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum WarningCode {
//     WT000
// }

impl DiagnosticCode for ErrorCode {
    fn message(&self) -> &'static str {
        match self {
            // === Tokenizer Errors === //
            
            // Numeric Literals
            Self::ET001 => "Numeric literal contains multiple decimal points",
            Self::ET002 => "Identifiers cannot immediately follow a numeric literal",
            Self::ET003 => "Numeric literal cannot end with a trailing decimal point",
            
            // Lexing / Characters
            Self::ET004 => "Unexpected or unrecognized character in source",
            Self::ET005 => "Character literal must contain exactly one character",
            Self::ET006 => "Character literal cannot be empty",
            Self::ET007 => "Unterminated character literal: missing closing single quote",
            Self::ET008 => "Unterminated character literal: closing quote is being escaped",
            
            // Escape Sequences & Encoding
            Self::ET009 => "Unknown or invalid escape sequence",
            Self::ET010 => "Invalid octal character escape",
            Self::ET011 => "Invalid hexadecimal character escape",
            
            // String Literals
            Self::ET012 => "Unterminated string literal: missing closing double quote",

            // === Parser Errors === //
            Self::EP001 => "Expected array size inside brackets",
            Self::EP002 => "Expected closing bracket after array size",
            Self::EP003 => "Expected a name after type in variable declaration",
            Self::EP004 => "Expected a value after assignment in variable declaration",
            Self::EP005 => "Expected a semicolon after variable declaration",
            Self::EP006 => "Expected a either a semi colon or an assignment after name in variable declaration",
            Self::EP007 => "Expected a condition after if keyword",
            Self::EP008 => "Expected a open curly brace after if keyword",
            Self::EP009 => "Expected a open curly brace after else keyword",
            Self::EP010 => "Expected a closing curly brace to close the body",
            Self::EP011 => "Expected a condition after if keyword",
            Self::EP012 => "Expected a open curly brace after if keyword",
            Self::EP013 => "Expected either arguments or an assignment after identifier",
            Self::EP014 => "Expected a closing parenthesis for function call",
            Self::EP015 => "Expected only expression as arguments for function call",
            Self::EP016 => "Expected a closing parenthesis after comma or another argument",
            Self::EP017 => "Expected a semicolon after",
            Self::EP018 => "Expected an expression after assignment operator",
            Self::EP019 => "Expected a semicolon after assignment operator",
            Self::EP020 => "Expected a semicolon after loop control keyword",
            Self::EP021 => "Expected an expression or a semicolon after return keyword",
            Self::EP022 => "Expected a semicolon after return statement",
            
        }
    }
}

// impl DiagnosticCode for WarningCode {
//     fn message(&self) -> &'static str {
//         match self {
//             WarningCode::WT000 => "Currently there are no warning"
//         }
//     }
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DiagnosticKind {
    Error(ErrorCode),
    // Warning(WarningCode),
}

impl DiagnosticKind {
    pub fn kind_to_str(&self) -> &'static str {
        match self {
            Self::Error(_) => "error",
            // Self::Warning(_) => "warning"
        }
    }

    pub fn code_to_string(&self) -> String {
        match self {
            Self::Error(code) => code.code_str(),
            // Self::Warning(code) => code.code_str(),
        }
    }


    pub fn message(&self) -> &'static str {
        match self {
            Self::Error(code) => code.message(),
            //Self::Warning(code) => code.message(),
        }
    }

}

#[derive(Debug, PartialEq)]
pub struct DiagnosticInfo<'a> {
    pub filename: &'a str,
    pub line: usize,
    pub column: usize
}

pub struct Diagnostic<'a> {
    pub kind: DiagnosticKind,
    pub info: DiagnosticInfo<'a>,
    pub hint: Option<&'a str>
}

impl<'a> Diagnostic<'a> {
    pub fn print(&self) {
        let red: &str = "\x1b[31;1m";
        let cyan: &str = "\x1b[36m";
        let yellow: &str = "\x1b[33m";
        let bold: &str = "\x1b[1m";
        let reset: &str = "\x1b[0m";

        let kind: &str = self.kind.kind_to_str();
        let code: String = self.kind.code_to_string();
        let message: &str = self.kind.message();

        println!("{red}{kind}[{code}]{reset}: {bold}{message}{reset}");
        
        println!("{cyan}  -->{reset} {}:{}:{}", self.info.filename, self.info.line, self.info.column);

        if let Ok(line_content) = self.read_line(self.info.line) {
            let line_num_str: String = self.info.line.to_string();
            let gutter_width: usize = line_num_str.len();
            let gutter_padding: String = " ".repeat(gutter_width);

            println!("{cyan} {} |{reset}", gutter_padding);

            println!("{cyan} {} |{reset} {}", line_num_str, line_content);

            print!("{cyan} {} |{reset} ", gutter_padding);
            
            for _ in 0..(self.info.column - gutter_width) {
                print!(" ");
            }

            if let Some(h) = &self.hint {
                println!("{yellow}^__ {}{reset}", h);
            } 
            else {
                println!("{yellow}^{reset}");
            }

            println!("{cyan} {} |{reset}", gutter_padding);
        }
    }

    fn read_line(&self, line: usize) -> io::Result<String> {
        let file: File = File::open(&self.info.filename)?;
        let reader: BufReader<File> = BufReader::new(file);

        reader.lines().nth(line - 1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Line not found"))?
    }
}