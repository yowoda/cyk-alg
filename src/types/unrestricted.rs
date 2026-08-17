use crate::{
    grammar::GrammarType,
    rules::{RuleCastingError, RuleType},
    symbols::SymbolId,
};

#[derive(Clone, Debug)]
pub struct UnrestrictedRule {
    pub left: Vec<SymbolId>,
    pub right: Vec<SymbolId>,
}

impl RuleType for UnrestrictedRule {
    fn into_unrestricted(self) -> Self {
        self
    }

    fn try_cast(rule: UnrestrictedRule) -> Result<Self, RuleCastingError> {
        Ok(rule)
    }
}

pub type UnrestrictedGrammar = GrammarType<UnrestrictedRule>;
