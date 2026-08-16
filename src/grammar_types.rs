use crate::{grammar::{Grammar, GrammarParsingError}, rules::Rule};

pub enum ContextFreeCastingError {
    LeftSideExactlyOneNonTerminal
}

pub enum GrammarCastingError {
    NotContextFree(Rule, ContextFreeCastingError),
    GrammarCouldNotBeParsed(GrammarParsingError)
}

pub trait GrammarType: Sized {
    fn try_cast(grammar: Grammar) -> Result<Self, GrammarCastingError>;
}

