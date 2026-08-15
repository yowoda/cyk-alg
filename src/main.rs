mod rules;
mod grammar;

use std::println;

use grammar::Grammar;

use crate::{
    grammar::GrammarParsingError::{DuplicateSymbolDeclared, RuleError, UnknownStartSymbol}, 
    rules::RuleParsingError::{EmptyLeftSide, EmptyRightSide, MultipleArrowMapping, UnknownSymbol}
};

fn main() {
    let grammar = Grammar::new(
        "a, b, c, d",
        "S, A",
        "S",
        "S -> A, A -> e"
    );
    match grammar {
        Ok(_) => (),
        Err(grammar_error) => match grammar_error {
            DuplicateSymbolDeclared => println!("All Terminals and Non-Terminals must have a unique representation"),
            UnknownStartSymbol => println!("The start symbol is not referencing a declared Non-Terminal"),
            RuleError(rule, rule_error) => match rule_error {
                EmptyLeftSide => println!("Rule '{}' does not have a left side", rule),
                EmptyRightSide => println!("Rule '{}' does not have a right side", rule),
                MultipleArrowMapping => println!("Rule '{}' contains multiple mappings", rule),
                UnknownSymbol(symbol) => println!("Rule '{}' contains unknown symbol '{}'", rule, symbol)
            }
        }
    }
    
}
