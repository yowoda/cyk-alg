use crate::rules::{Rule, RuleParsingError, Symbol, SymbolId, SymbolType};

use std::{collections::HashMap, println};

struct SymbolSpec {
    symbols: HashMap<SymbolId, Symbol>,
    symbol_id_mapping: HashMap<String, SymbolId>,
    next_id: usize,
    start_symbol: Option<SymbolId>,
}

impl SymbolSpec {
    pub fn get_symbol_id(&self, source: &str) -> Option<SymbolId> {
        self.symbol_id_mapping.get(source).map(|&s| s)
    }

    pub fn set_start_symbol(&mut self, text: &str) -> Result<(), GrammarParsingError> {
        match self.get_symbol_id(text) {
            Some(id) => {
                if self.symbols.get(&id).unwrap().get_type() == SymbolType::Terminal {
                    return Err(GrammarParsingError::UnknownStartSymbol)
                }

                self.start_symbol = Some(id);
                Ok(())
            }
            None => Err(GrammarParsingError::UnknownStartSymbol)
        }
    }

    pub fn symbol_exists(&self, text: &str) -> bool {
        self.symbol_id_mapping.contains_key(text)
    }

    fn add_symbols(&mut self, source: &str, stype: SymbolType) -> Result<(), GrammarParsingError> {
        for token in source.split(",").map(|s| s.trim()) {
            self.add_symbol(token, stype)?;
        }

        Ok(())
    }

    fn add_symbol(&mut self, text: &str, stype: SymbolType) -> Result<SymbolId, GrammarParsingError> {
        if self.symbol_exists(text) {
            return Err(GrammarParsingError::DuplicateSymbolDeclared)
        }

        let symbol_id = SymbolId { id: self.next_id };
        self.next_id += 1;
        let symbol = Symbol::new(text.to_string(), stype);
        self.symbol_id_mapping.insert(text.to_string(), symbol_id);
        self.symbols.insert(symbol_id, symbol);

        Ok(symbol_id)

    }

    pub fn new() -> Self {
        SymbolSpec {
            symbols: HashMap::new(),
            symbol_id_mapping: HashMap::new(),
            next_id: 0,
            start_symbol: Option::None
        }
    }
}

pub struct Grammar {
    symbol_spec: SymbolSpec,
    rules: Vec<Rule>
}

#[derive(Debug)]
pub enum GrammarParsingError {
    DuplicateSymbolDeclared,
    UnknownStartSymbol,
    RuleError(String, RuleParsingError)
}

impl Grammar {
    fn add_rule(&mut self, source: &str) -> Result<(), RuleParsingError> {
        let mut left_symbols = Vec::new();
        let mut right_symbols = Vec::new();

        let mut curr_symbols = &mut left_symbols;

        let tokens = source.split_whitespace();

        let mut arrow_counter = 0;

        for token in tokens {
            if token == "->" {
                if left_symbols.is_empty() {
                    return Err(RuleParsingError::EmptyLeftSide);
                }

                if arrow_counter == 1 {
                    return Err(RuleParsingError::MultipleArrowMapping);
                }

                curr_symbols = &mut right_symbols;
                arrow_counter += 1;
            } else {
                match self.symbol_spec.get_symbol_id(token) {
                    Some(id) => curr_symbols.push(id),
                    None => return Err(RuleParsingError::UnknownSymbol(token.to_string()))
                }
            }
        }

        if right_symbols.is_empty() {
            return Err(RuleParsingError::EmptyRightSide);
        }

        self.rules.push(Rule {left: left_symbols, right: right_symbols});

        Ok(())
    }

    pub fn add_rules(&mut self, source: &str) -> Result<(), GrammarParsingError> {
        let tokens = source.split(",").map(|s| s.trim());

        for token in tokens {
            self.add_rule(token).map_err(
                |error| GrammarParsingError::RuleError(token.to_string(), error)
            )?;
        }

        Ok(())
    }

    pub fn new(
        terminals_source: &str, non_terminals_source: &str,
        start_symbol_source: &str, source: &str
    ) -> Result<Self, GrammarParsingError> {
        let mut symbol_spec = SymbolSpec::new();
        symbol_spec.add_symbols(terminals_source, SymbolType::Terminal)?;
        symbol_spec.add_symbols(non_terminals_source, SymbolType::NonTerminal)?;
        symbol_spec.set_start_symbol(start_symbol_source)?;

        let mut grammar = Grammar {
            symbol_spec: symbol_spec,
            rules: Vec::new()
        };

        grammar.add_rules(source)?;

        Ok(grammar)
    }

}