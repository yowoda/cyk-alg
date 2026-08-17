use crate::{
    grammar::GrammarType,
    rules::{RuleCastingError, RuleType},
    symbols::SymbolId,
    types::{context_free::ContextFreeRule, unrestricted::UnrestrictedRule},
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
    fn into_unrestricted(self) -> UnrestrictedRule {
        UnrestrictedRule {
            left: vec![self.left],
            right: match self.right {
                RightCnfRule::NonTerminals(id1, id2) => vec![id1, id2],
                RightCnfRule::Terminal(id) => vec![id],
            },
        }
    }

    fn try_cast(rule: UnrestrictedRule) -> Result<Self, RuleCastingError> {
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

pub type CnfGrammar = GrammarType<CnfRule>;
