use crate::{
    grammar::GrammarType,
    rules::{RuleCastingError, RuleType},
    symbols::{SymbolId, SymbolSpec},
    types::{context_free::ContextFreeRule, unrestricted::Rule},
};

enum RightCnfRule {
    Terminal(SymbolId),
    NonTerminals(SymbolId, SymbolId),
}

pub struct CnfRule {
    left: SymbolId,
    right: RightCnfRule,
}

impl RuleType for CnfRule {
    fn into_general(self) -> Rule {
        Rule {
            left: vec![self.left],
            right: match self.right {
                RightCnfRule::NonTerminals(id1, id2) => vec![id1, id2],
                RightCnfRule::Terminal(id) => vec![id],
            },
        }
    }

    fn try_cast(rule: Rule) -> Result<Self, RuleCastingError> {
        let cfg_rule = ContextFreeRule::try_cast(rule)?;

        let left = cfg_rule.left;
        let right = cfg_rule.right.clone();

        match right.len() {
            1 => {
                if !matches!(right[0], SymbolId::Terminal(_)) {
                    return Err(RuleCastingError::NotInCNF);
                }

                return Ok(CnfRule {
                    left: left,
                    right: RightCnfRule::Terminal(right[0]),
                });
            }
            2 => {
                if !matches!(
                    right[..],
                    [SymbolId::NonTerminal(_), SymbolId::NonTerminal(_)]
                ) {
                    return Err(RuleCastingError::NotInCNF);
                }

                return Ok(CnfRule {
                    left: left,
                    right: RightCnfRule::NonTerminals(right[0], right[1]),
                });
            }
            _ => return Err(RuleCastingError::NotInCNF),
        }
    }
}

pub struct CnfGrammar {
    symbol_spec: SymbolSpec,
    rules: Vec<CnfRule>,
}

impl GrammarType for CnfGrammar {
    type Rule = CnfRule;

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
