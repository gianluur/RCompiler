use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct SourceLiteral(pub usize);

impl SourceLiteral {
    pub fn dummy() -> SourceLiteral {
        SourceLiteral(0)
    }
}

impl fmt::Display for SourceLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Interner {
    map: HashMap<String, SourceLiteral>,
    strings: Vec<String>,
}

impl Interner {
    pub fn new() -> Interner {
        let mut i = Interner {
            map: HashMap::new(),
            strings: Vec::new(),
        };
        // Always ensure index 0 is the empty string
        i.intern(""); 
        i
    }

    pub fn intern(&mut self, name: &str) -> SourceLiteral {
        if let Some(&literal) = self.map.get(name) {
            return literal;
        }
        
        let literal: SourceLiteral = SourceLiteral(self.strings.len());
        self.strings.push(name.to_string());
        self.map.insert(name.to_string(), literal);
        literal
    }

    pub fn lookup(&self, symbol: SourceLiteral) -> &str {
        &self.strings[symbol.0]
    }

}
