mod grammar;
mod rules;
mod symbols;
mod types;

use std::println;

use crate::{
    grammar::{
        GrammarError::{CastingError, ParsingError},
        GrammarParsingError::{RuleCouldNotBeParsed, SymbolError},
    },
    rules::{
        RuleCastingError::{NotContextFree, NotInCNF},
        RuleParsingError::{
            EmptyLeftSide, EmptyRightSide, InvalidUseOfAlternationOperator, MultipleArrowMapping,
            UnknownSymbol,
        },
    },
    symbols::SymbolSpecError::{
        DuplicateSymbolDeclared, StartSymbolMustBeNonTerminal, UnknownStartSymbol,
    },
    types::cnf::CnfGrammar,
};

fn main() {
    let grammar = CnfGrammar::parse("a, b, c, d", "S, A", "S", "S -> A a , A -> b");

    match grammar {
        Ok(_) => (),
        Err(grammar_error) => match grammar_error {
            ParsingError(grammar_parsing_error) => match grammar_parsing_error {
                RuleCouldNotBeParsed(source, rule_parsing_error) => match rule_parsing_error {
                    EmptyLeftSide => println!("Rule '{}' does not have a left side", source),
                    EmptyRightSide => println!("Rule '{}' does not have a right side", source),
                    MultipleArrowMapping => {
                        println!("Rule '{}' contains multiple mappings", source)
                    }
                    UnknownSymbol(symbol) => {
                        println!("Rule '{}' contains unknown symbol '{}'", source, symbol)
                    }
                    InvalidUseOfAlternationOperator => println!(
                        "Rule '{}' makes invalid use of alternation operator '|'",
                        source
                    ),
                },
                SymbolError(symbol_error) => match symbol_error {
                    DuplicateSymbolDeclared(sym) => {
                        println!("Symbol '{}' has already been declared", sym)
                    }
                    UnknownStartSymbol(sym) => println!(
                        "Start symbol '{}' is not referencing a declared Non-Terminal",
                        sym
                    ),
                    StartSymbolMustBeNonTerminal(sym) => println!(
                        "Start symbol '{}' should refer to a Non-Terminal, not a terminal",
                        sym
                    ),
                },
            },
            CastingError(rule, rule_casting_error) => match rule_casting_error {
                NotContextFree => println!("Rule is not context free"),
                NotInCNF => println!("Rule is not in CNF"),
            },
        },
    }
}
