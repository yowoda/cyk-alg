use crate::{grammar::{Grammar, SymbolSpec}, grammar_types::{ContextFreeCastingError, GrammarCastingError, GrammarType}, rules::{SymbolId, SymbolType}};

struct ContextFreeRule {
    pub left: SymbolId,
    pub right: Vec<SymbolId>
}

pub struct ContextFreeGrammar {
    symbol_spec: SymbolSpec,
    rules: Vec<ContextFreeRule>
}

impl GrammarType for ContextFreeGrammar {
    fn try_cast(grammar: Grammar) -> Result<Self, GrammarCastingError> {
        let mut rules = Vec::new();
        
        for rule in grammar.rules() {
            let left = rule.left.clone();
            
            if left.len() != 1 || grammar.symbol_spec().get_symbol_by_id(left[0]).unwrap().stype() == SymbolType::Terminal {
                return Err(GrammarCastingError::NotContextFree(rule.clone(), ContextFreeCastingError::LeftSideExactlyOneNonTerminal));
            }

            rules.push(ContextFreeRule { left: left[0], right: rule.right.clone() });
        }

        let cfg = ContextFreeGrammar {
            symbol_spec: grammar.into_symbol_spec(),
            rules: rules
        };

        Ok(cfg)
    }
}