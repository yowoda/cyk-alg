use crate::{
    grammar::GrammarType,
    rules::{RuleCastingError, RuleType},
    symbols::{SymbolId, SymbolSpec},
};

#[derive(Clone, Debug)]
pub struct Rule {
    pub left: Vec<SymbolId>,
    pub right: Vec<SymbolId>,
}

impl RuleType for Rule {
    fn into_general(self) -> Self {
        self
    }

    fn try_cast(rule: Rule) -> Result<Self, RuleCastingError> {
        Ok(rule)
    }
}

pub struct Grammar {
    symbol_spec: SymbolSpec,
    rules: Vec<Rule>,
}

impl GrammarType for Grammar {
    type Rule = Rule;

    fn new(symbol_spec: SymbolSpec, rules: Vec<Rule>) -> Self {
        return Grammar { symbol_spec, rules };
    }

    fn rules_mut(&mut self) -> &mut Vec<Rule> {
        &mut self.rules
    }

    fn symbol_spec(&self) -> &SymbolSpec {
        &self.symbol_spec
    }

    fn symbol_spec_mut(&mut self) -> &mut SymbolSpec {
        &mut self.symbol_spec
    }

    fn into_general(self) -> Grammar {
        self
    }

    fn into_parts(self) -> (SymbolSpec, Vec<Self::Rule>) {
        (self.symbol_spec, self.rules)
    }

    fn try_cast<G: GrammarType>(grammar: G) -> Result<Self, RuleCastingError> {
        Ok(grammar.into_general())
    }
}
