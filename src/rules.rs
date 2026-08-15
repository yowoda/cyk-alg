#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId {
    pub id: usize
}

pub struct Rule {
    pub left: Vec<SymbolId>,
    pub right: Vec<SymbolId>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Terminal,
    NonTerminal
}

pub struct Symbol {
    text: String,
    stype: SymbolType
}

impl Symbol {
    pub fn new(text: String, stype: SymbolType) -> Self {
        Symbol {text, stype}
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_type(&self) -> SymbolType {
        self.stype
    }
}

#[derive(Debug)]
pub enum RuleParsingError {
    EmptyLeftSide,
    EmptyRightSide,
    MultipleArrowMapping,
    InvalidUseOfAlternationOperator,
    UnknownSymbol(String)
}