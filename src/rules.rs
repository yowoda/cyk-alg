#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId {
    pub id: usize
}

#[derive(Clone)]
pub struct Rule {
    pub left: Vec<SymbolId>,
    pub right: Vec<SymbolId>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Terminal,
    NonTerminal
}

#[derive(Clone)]
pub struct Symbol {
    text: String,
    stype: SymbolType
}

impl Symbol {
    pub fn new(text: String, stype: SymbolType) -> Self {
        Symbol {text, stype}
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn stype(&self) -> SymbolType {
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