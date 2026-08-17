use crate::{
    grammar::GrammarType,
    rules::{RuleCastingError, RuleType},
    symbols::{SymbolId, SymbolSpec},
    types::unrestricted::Rule,
};

#[derive(Debug)]
pub struct ContextFreeRule {
    pub left: SymbolId,
    pub right: Vec<SymbolId>,
}

impl RuleType for ContextFreeRule {
    fn into_general(self) -> Rule {
        Rule {
            left: vec![self.left],
            right: self.right,
        }
    }

    fn try_cast(rule: Rule) -> Result<Self, RuleCastingError> {
        let left = rule.left.clone();

        if left.len() != 1 || !matches!(left[0], SymbolId::NonTerminal(_)) {
            return Err(RuleCastingError::NotContextFree);
        }

        Ok(ContextFreeRule {
            left: left[0],
            right: rule.right.clone(),
        })
    }
}

pub struct ContextFreeGrammar {
    symbol_spec: SymbolSpec,
    rules: Vec<ContextFreeRule>,
}

impl GrammarType for ContextFreeGrammar {
    type Rule = ContextFreeRule;

    fn new(symbol_spec: SymbolSpec, rules: Vec<Self::Rule>) -> Self {
        Self { symbol_spec, rules }
    }

    fn rules_mut(&mut self) -> &mut Vec<Self::Rule> {
        &mut self.rules
    }

    fn symbol_spec(&self) -> &SymbolSpec {
        &self.symbol_spec
    }

    fn symbol_spec_mut(&mut self) -> &mut SymbolSpec {
        &mut self.symbol_spec
    }

    fn into_parts(self) -> (SymbolSpec, Vec<Self::Rule>) {
        (self.symbol_spec, self.rules)
    }
}
