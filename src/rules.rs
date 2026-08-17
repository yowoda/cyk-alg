use crate::types::unrestricted::UnrestrictedRule;

pub trait RuleType: Sized {
    fn into_unrestricted(self) -> UnrestrictedRule;

    fn try_cast(rule: UnrestrictedRule) -> Result<Self, RuleCastingError>;
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
    RuleCastingError(UnrestrictedRule, RuleCastingError),
}
