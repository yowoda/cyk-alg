use crate::types::unrestricted::Rule;

pub trait RuleType: Sized {
    fn into_general(self) -> Rule;

    fn try_cast(rule: Rule) -> Result<Self, RuleCastingError>;
}

#[derive(Debug)]
pub enum RuleCastingError {
    NotContextFree,
    NotInCNF,
}

#[derive(Debug)]
pub enum RuleParsingError {
    EmptyLeftSide,
    EmptyRightSide,
    MultipleArrowMapping,
    InvalidUseOfAlternationOperator,
    UnknownSymbol(String),
}

#[derive(Debug)]
pub enum RuleError {
    RuleParsingError(String, RuleParsingError),
    RuleCastingError(Rule, RuleCastingError),
}
