use crate::{
    rules::{
        RuleCastingError, RuleError,
        RuleParsingError::{self},
        RuleType,
    },
    symbols::{SymbolId, SymbolSpec, SymbolSpecError, SymbolType},
    types::unrestricted::{Grammar, Rule},
};

#[derive(Debug)]
pub enum GrammarParsingError {
    RuleCouldNotBeParsed(String, RuleParsingError),
    SymbolError(SymbolSpecError),
}

pub enum GrammarError {
    ParsingError(GrammarParsingError),
    CastingError(Rule, RuleCastingError),
}

impl From<SymbolSpecError> for GrammarError {
    fn from(err: SymbolSpecError) -> Self {
        GrammarError::ParsingError(GrammarParsingError::SymbolError(err))
    }
}

impl From<GrammarParsingError> for GrammarError {
    fn from(err: GrammarParsingError) -> Self {
        GrammarError::ParsingError(err)
    }
}

impl From<RuleError> for GrammarError {
    fn from(err: RuleError) -> Self {
        match err {
            RuleError::RuleParsingError(source, err) => {
                GrammarError::ParsingError(GrammarParsingError::RuleCouldNotBeParsed(source, err))
            }
            RuleError::RuleCastingError(rule, err) => GrammarError::CastingError(rule, err),
        }
    }
}

pub trait GrammarType: Sized {
    type Rule: RuleType;

    fn new(symbol_spec: SymbolSpec, rules: Vec<Self::Rule>) -> Self;

    fn rules_mut(&mut self) -> &mut Vec<Self::Rule>;

    fn symbol_spec(&self) -> &SymbolSpec;

    fn symbol_spec_mut(&mut self) -> &mut SymbolSpec;

    fn parse_left_rule<'a, I>(&self, tokens: &mut I) -> Result<Vec<SymbolId>, RuleParsingError>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut symbols = Vec::new();

        while let Some(token) = tokens.next()
            && token != "->"
        {
            if token == "|" {
                return Err(RuleParsingError::InvalidUseOfAlternationOperator);
            } else {
                match self.symbol_spec().get_symbol_id(token) {
                    Some(id) => symbols.push(id),
                    None => return Err(RuleParsingError::UnknownSymbol(token.to_string())),
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
        I: Iterator<Item = &'a str>,
    {
        let mut symbols = Vec::new();

        let mut token = match tokens.next() {
            Some("|") => return Err(RuleParsingError::InvalidUseOfAlternationOperator),
            None => return Ok(symbols),
            Some(source) => source,
        };

        loop {
            if token == "->" {
                return Err(RuleParsingError::MultipleArrowMapping);
            }

            match self.symbol_spec().get_symbol_id(token) {
                Some(id) => symbols.push(id),
                None => return Err(RuleParsingError::UnknownSymbol(token.to_string())),
            }

            match tokens.next() {
                Some("|") | None => return Ok(symbols),
                Some(source) => {
                    token = source;
                }
            }
        }
    }

    fn parse_right_rule<'a, I>(
        &self,
        tokens: &mut I,
    ) -> Result<Vec<Vec<SymbolId>>, RuleParsingError>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut right_side = Vec::new();

        while let symbols = self.parse_right_subrule(tokens)?
            && !symbols.is_empty()
        {
            right_side.push(symbols);
        }

        Ok(right_side)
    }

    fn parse_rule(&self, source: &str) -> Result<Vec<Rule>, RuleParsingError> {
        let mut tokens = source.split_whitespace();
        let left_symbols = self.parse_left_rule(&mut tokens)?;
        let right_side = self.parse_right_rule(&mut tokens)?;

        if right_side.is_empty() {
            return Err(RuleParsingError::EmptyRightSide);
        }

        let mut rules = Vec::new();

        for right_symbols in right_side {
            rules.push(Rule {
                left: left_symbols.clone(),
                right: right_symbols,
            });
        }

        Ok(rules)
    }

    fn add_rule(&mut self, source: &str) -> Result<(), RuleError> {
        let rules = self
            .parse_rule(source)
            .map_err(|err| RuleError::RuleParsingError(source.to_string(), err))?;

        for rule in rules {
            self.rules_mut().push(
                Self::Rule::try_cast(rule.clone())
                    .map_err(|err| RuleError::RuleCastingError(rule, err))?,
            );
        }

        Ok(())
    }

    fn add_rules(&mut self, source: &str) -> Result<(), RuleError> {
        let tokens = source.split(",").map(|s| s.trim());

        for token in tokens {
            self.add_rule(token)?;
        }

        Ok(())
    }

    fn parse(
        terminals_source: &str,
        non_terminals_source: &str,
        start_symbol_source: &str,
        source: &str,
    ) -> Result<Self, GrammarError> {
        let symbol_spec = SymbolSpec::new();
        let mut grammar = Self::new(symbol_spec, Vec::new());

        let symbol_spec = grammar.symbol_spec_mut();

        symbol_spec.add_symbols(terminals_source, SymbolType::Terminal)?;
        symbol_spec.add_symbols(non_terminals_source, SymbolType::NonTerminal)?;
        symbol_spec.set_start_symbol(start_symbol_source)?;
        symbol_spec.set_empty_string("_")?;

        grammar.add_rules(source)?;

        Ok(grammar)
    }

    fn into_parts(self) -> (SymbolSpec, Vec<Self::Rule>);

    fn into_general(self) -> Grammar {
        let (spec, rules) = self.into_parts();
        let mut general_rules = Vec::new();

        for rule in rules {
            general_rules.push(rule.into_general());
        }

        Grammar::new(spec, general_rules)
    }

    fn try_cast<G: GrammarType>(grammar: G) -> Result<Self, RuleCastingError> {
        let general_grammar = grammar.into_general();

        let (spec, rules) = general_grammar.into_parts();

        let mut cast_rules = Vec::new();

        for rule in rules {
            cast_rules.push(Self::Rule::try_cast(rule)?);
        }

        Ok(Self::new(spec, cast_rules))
    }
}
