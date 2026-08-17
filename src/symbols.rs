use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolId {
    Terminal(usize),
    NonTerminal(usize),
    Empty(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Terminal,
    NonTerminal,
    Empty,
}

#[derive(Clone)]
pub struct Symbol {
    text: String,
}

impl Symbol {
    pub fn new(text: String) -> Self {
        Symbol { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

pub struct SymbolSpec {
    symbols: HashMap<SymbolId, Symbol>,
    symbol_id_mapping: HashMap<String, SymbolId>,
    next_id: usize,
    start_symbol: Option<SymbolId>,
    empty_string: Option<String>,
}

#[derive(Debug)]
pub enum SymbolSpecError {
    StartSymbolMustBeNonTerminal(String),
    UnknownStartSymbol(String),
    DuplicateSymbolDeclared(String),
}

impl SymbolSpec {
    pub fn get_symbol_id(&self, source: &str) -> Option<SymbolId> {
        self.symbol_id_mapping.get(source).map(|&s| s)
    }

    pub fn get_symbol_by_id(&self, symbol_id: SymbolId) -> Option<&Symbol> {
        self.symbols.get(&symbol_id)
    }

    pub fn set_start_symbol(&mut self, text: &str) -> Result<(), SymbolSpecError> {
        match self.get_symbol_id(text) {
            Some(id) => {
                if !matches!(id, SymbolId::NonTerminal(_)) {
                    return Err(SymbolSpecError::StartSymbolMustBeNonTerminal(
                        text.to_string(),
                    ));
                }

                self.start_symbol = Some(id);
                Ok(())
            }
            None => Err(SymbolSpecError::UnknownStartSymbol(text.to_string())),
        }
    }

    pub fn set_empty_string(&mut self, source: &str) -> Result<(), SymbolSpecError> {
        if self.symbol_exists(source) {
            return Err(SymbolSpecError::DuplicateSymbolDeclared(source.to_string()));
        }

        if let Some(s) = self.empty_string.take() {
            self.delete_symbol(&s);
        }

        self.empty_string = Some(source.to_string());
        let _ = self.add_symbol(source, SymbolType::Empty);

        Ok(())
    }

    pub fn symbol_exists(&self, text: &str) -> bool {
        self.symbol_id_mapping.contains_key(text)
    }

    pub fn add_symbols(&mut self, source: &str, stype: SymbolType) -> Result<(), SymbolSpecError> {
        for token in source.split(",").map(|s| s.trim()) {
            self.add_symbol(token, stype)?;
        }

        Ok(())
    }

    pub fn add_symbol(
        &mut self,
        text: &str,
        stype: SymbolType,
    ) -> Result<SymbolId, SymbolSpecError> {
        if self.symbol_exists(text) {
            return Err(SymbolSpecError::DuplicateSymbolDeclared(text.to_string()));
        }

        let symbol_id = match stype {
            SymbolType::Terminal => SymbolId::Terminal(self.next_id),
            SymbolType::NonTerminal => SymbolId::NonTerminal(self.next_id),
            SymbolType::Empty => SymbolId::Empty(self.next_id),
        };
        self.next_id += 1;
        let symbol = Symbol::new(text.to_string());
        self.symbol_id_mapping.insert(text.to_string(), symbol_id);
        self.symbols.insert(symbol_id, symbol);

        Ok(symbol_id)
    }

    fn delete_symbol(&mut self, source: &str) -> Option<String> {
        let id = match self.get_symbol_id(source) {
            Some(id) => id,
            None => return None,
        };

        match self.start_symbol {
            Some(start_id) => {
                if start_id == id {
                    self.start_symbol = Option::None
                }
            }
            None => (),
        }

        self.symbol_id_mapping.remove(source);
        self.symbols.remove(&id);

        Some(source.to_string())
    }

    pub fn new() -> Self {
        SymbolSpec {
            symbols: HashMap::new(),
            symbol_id_mapping: HashMap::new(),
            next_id: 0,
            start_symbol: Option::None,
            empty_string: Option::None,
        }
    }
}
