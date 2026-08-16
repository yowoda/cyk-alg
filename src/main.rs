mod rules;
mod grammar;
mod grammar_types;
mod types;

use std::println;

use grammar::Grammar;

use crate::{
    grammar::GrammarParsingError::{DuplicateSymbolDeclared, RuleError, UnknownStartSymbol}, grammar_types::{ContextFreeCastingError::{LeftSideExactlyOneNonTerminal}, GrammarCastingError, GrammarType}, rules::RuleParsingError::{EmptyLeftSide, EmptyRightSide, InvalidUseOfAlternationOperator, MultipleArrowMapping, UnknownSymbol}, types::context_free::ContextFreeGrammar
};

fn main() {
    let grammar = parse::<ContextFreeGrammar>(
        "a, b, c, d",
        "S, A",
        "S",
        "S -> a b c d b c d , A -> b"
    );
    match grammar {
        Ok(_) => (),
        Err(casting_error) => match casting_error {
            GrammarCastingError::NotContextFree(rule, cfg_error) => match cfg_error {
                LeftSideExactlyOneNonTerminal => println!("Left side of CFG must consist of exactly one Non-terminal")
            },
            GrammarCastingError::GrammarCouldNotBeParsed(grammar_error) => match grammar_error {
                DuplicateSymbolDeclared => println!("All Terminals and Non-Terminals must have a unique representation"),
                UnknownStartSymbol => println!("The start symbol is not referencing a declared Non-Terminal"),
                RuleError(rule, rule_error) => match rule_error {
                    EmptyLeftSide => println!("Rule '{}' does not have a left side", rule),
                    EmptyRightSide => println!("Rule '{}' does not have a right side", rule),
                    MultipleArrowMapping => println!("Rule '{}' contains multiple mappings", rule),
                    UnknownSymbol(symbol) => println!("Rule '{}' contains unknown symbol '{}'", rule, symbol),
                    InvalidUseOfAlternationOperator => println!("Rule '{}' makes invalid use of alternation operator '|'", rule)
                }
            }
        }
    };
    
}

fn parse<G: GrammarType>(
    terminals_source: &str,
    non_terminals_source: &str,
    start_symbol_source: &str,
    rules_source: &str
) -> Result<G, GrammarCastingError> {
    let grammar = match Grammar::new(
        terminals_source, non_terminals_source, start_symbol_source, rules_source
    ) {
        Ok(g) => g,
        Err(grammar_error) => return Err(GrammarCastingError::GrammarCouldNotBeParsed(grammar_error))
    };

    Ok(G::try_cast(grammar)?)
}