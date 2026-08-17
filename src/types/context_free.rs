use crate::{
    grammar::GrammarType,
    rules::{RuleCastingError, RuleType},
    symbols::SymbolId,
    types::unrestricted::UnrestrictedRule,
};

#[derive(Debug)]
pub struct ContextFreeRule {
    pub left: SymbolId,
    pub right: Vec<SymbolId>,
}

impl RuleType for ContextFreeRule {
    fn into_unrestricted(self) -> UnrestrictedRule {
        UnrestrictedRule {
            left: vec![self.left],
            right: self.right,
        }
    }

    fn try_cast(rule: UnrestrictedRule) -> Result<Self, RuleCastingError> {
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

pub type ContextFreeGrammar = GrammarType<ContextFreeRule>;
