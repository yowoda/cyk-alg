use crate::rules::{Rule, RuleParsingError, Symbol, SymbolId, SymbolType};

use std::collections::HashMap;

#[derive(Clone)]
pub struct SymbolSpec {
    symbols: HashMap<SymbolId, Symbol>,
    symbol_id_mapping: HashMap<String, SymbolId>,
    next_id: usize,
    start_symbol: Option<SymbolId>,
}

impl SymbolSpec {
    pub fn get_symbol_id(&self, source: &str) -> Option<SymbolId> {
        self.symbol_id_mapping.get(source).map(|&s| s)
    }

    pub fn get_symbol_by_id(&self, symbol_id: SymbolId) -> Option<&Symbol> {
        self.symbols.get(&symbol_id)
    }

    pub fn set_start_symbol(&mut self, text: &str) -> Result<(), GrammarParsingError> {
        match self.get_symbol_id(text) {
            Some(id) => {
                if self.symbols.get(&id).unwrap().stype() == SymbolType::Terminal {
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
    fn parse_left_rule<'a, I>(&self, tokens: &mut I) -> Result<Vec<SymbolId>, RuleParsingError> 
    where 
        I: Iterator<Item = &'a str> 
    {
        let mut symbols = Vec::new();

        while let Some(token) = tokens.next() && token != "->" {
            if token == "|" {
                return Err(RuleParsingError::InvalidUseOfAlternationOperator);
            } else {
                match self.symbol_spec.get_symbol_id(token) {
                    Some(id) => symbols.push(id),
                    None => return Err(RuleParsingError::UnknownSymbol(token.to_string()))
                }
            }
        }

        if symbols.is_empty() {
            return Err(RuleParsingError::EmptyLeftSide);
        }

        Ok(symbols)
    }

     fn parse_right_subrule<'a, I>(&self, tokens: &mut I) -> Result<Vec<SymbolId>, RuleParsingError>
    where
        I: Iterator<Item = &'a str>
    {
        let mut symbols = Vec::new();

        let mut token = match tokens.next() {
            Some("|") => return Err(RuleParsingError::InvalidUseOfAlternationOperator),
            None => return Ok(symbols),
            Some(source) => source
        };

        loop {
            if token == "->" {
                return Err(RuleParsingError::MultipleArrowMapping);
            }

            match self.symbol_spec.get_symbol_id(token) {
                Some(id) => symbols.push(id),
                None => return Err(RuleParsingError::UnknownSymbol(token.to_string()))
            }

            match tokens.next() {
                Some("|") | None => return Ok(symbols),
                Some(source) => {
                    token = source;
                },
            }
        }
    }

    fn parse_right_rule<'a, I>(&self, tokens: &mut I) -> Result<Vec<Vec<SymbolId>>, RuleParsingError>
    where
        I: Iterator<Item = &'a str>
    {
        let mut right_side = Vec::new();

        while let symbols = self.parse_right_subrule(tokens)? && !symbols.is_empty() {
            right_side.push(symbols);
        }

        Ok(right_side)
    }

    fn add_rule(&mut self, source: &str) -> Result<(), RuleParsingError> {
        let mut tokens = source.split_whitespace();
        let left_symbols = self.parse_left_rule(&mut tokens)?;
        let right_side = self.parse_right_rule(&mut tokens)?;

        if right_side.is_empty() {
            return Err(RuleParsingError::EmptyRightSide);
        }
        
        for right_symbols in right_side {
            self.rules.push(Rule {left: left_symbols.clone(), right: right_symbols});
        }

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

    pub fn rules(&self) -> &Vec<Rule> {
        &self.rules
    }

    pub fn symbol_spec(&self) -> &SymbolSpec {
        &self.symbol_spec
    }

    pub fn into_symbol_spec(self) -> SymbolSpec {
        self.symbol_spec
    }
}