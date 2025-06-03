use crate::token::{Literal, Token};
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "sexpr.pest"]
pub struct CoreLangParser;

impl std::str::FromStr for Literal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cons" => Ok(Literal::Cons),
            "car" => Ok(Literal::Car),
            "cdr" => Ok(Literal::Cdr),
            "if" => Ok(Literal::If),
            "lambda" => Ok(Literal::Lambda),
            "begin" => Ok(Literal::Begin),
            "define" => Ok(Literal::Define),
            "define-syntax" => Ok(Literal::DefineSyntax),
            "call/cc" => Ok(Literal::CallCc),
            _ => Err(String::from(
                "Failed to parse literal. Perhaps this is a word",
            )),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Cons => write!(f, "cons"),
            Literal::Car => write!(f, "car"),
            Literal::Cdr => write!(f, "cdr"),
            Literal::If => write!(f, "if"),
            Literal::Lambda => write!(f, "lambda"),
            Literal::Begin => write!(f, "begin"),
            Literal::Define => write!(f, "define"),
            Literal::DefineSyntax => write!(f, "define_syntax"),
            Literal::CallCc => write!(f, "call_cc"),
        }
    }
}

fn parse_pair(pair: Pair<Rule>) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    match pair.as_rule() {
        Rule::EOI
        | Rule::punct
        | Rule::sexpr
        | Rule::word
        | Rule::number
        | Rule::string
        | Rule::list
        | Rule::left_parenthesis
        | Rule::right_parenthesis => unreachable!(),
        Rule::program => {
            let mut result: Vec<Token> = Vec::new();

            let rule = pair.into_inner();
            rule.clone().for_each(|w| match w.as_rule() {
                Rule::sexpr => result.push(parse_sexpr(w).unwrap()),
                Rule::list => result.push(parse_list(w).unwrap()),
                Rule::word => {
                    let str: String = String::from(w.as_span().as_str());
                    result.push(Token::Word(str));
                }
                Rule::string => result.push(parse_string(w).unwrap()),
                Rule::number => {
                    let number: u64 = w
                        .as_span()
                        .as_str()
                        .parse::<u64>()
                        .expect("PARSE_ERROR: Failed to parse number");
                    result.push(Token::Number(number));
                }
                Rule::program | Rule::punct | Rule::left_parenthesis | Rule::right_parenthesis => {
                    unreachable!()
                }
                Rule::EOI => (),
            });

            Ok(result)
        }
    }
}

fn parse_word(word: Pair<Rule>) -> Result<Token, Box<dyn std::error::Error>> {
    let s: String = String::from(word.as_span().as_str());
    match Literal::from_str(&s) {
        Ok(v) => Ok(Token::Literal(v)),
        Err(_) => Ok(Token::Word(s)),
    }
}

fn parse_string(string: Pair<Rule>) -> Result<Token, Box<dyn std::error::Error>> {
    let s: &str = string.as_span().as_str();
    let result: String = strip_quotes(s).to_string();

    Ok(Token::String(result))
}

fn strip_quotes(s: &str) -> &str {
    let bytes = s.as_bytes();

    if bytes.len() >= 2 && bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"' {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn parse_number(word: Pair<Rule>) -> Result<Token, Box<dyn std::error::Error>> {
    let w: &str = word.as_span().as_str().trim();
    let number: u64 = w
        .parse::<u64>()
        .expect("PARSE_ERROR: Failed to parse number");

    Ok(Token::Number(number))
}

fn parse_sexpr(sexpr: Pair<Rule>) -> Result<Token, Box<dyn std::error::Error>> {
    let mut result: Vec<Token> = Vec::new();

    let rule = sexpr.into_inner();
    let mut words: Vec<Pair<Rule>> = rule.into_iter().collect();

    words.pop(); // right_parenthesis
    words.reverse(); // Reverse to pop left_parenthesis
    words.pop(); // left_parenthesis
    words.reverse(); // Reverse to fix the order

    for w in words {
        match w.as_rule() {
            Rule::sexpr => result.push(parse_sexpr(w)?),
            Rule::word => result.push(parse_word(w)?),
            Rule::number => result.push(parse_number(w)?),
            Rule::string => result.push(parse_string(w)?),
            Rule::list => result.push(parse_list(w)?),
            Rule::program | Rule::punct | Rule::left_parenthesis | Rule::right_parenthesis => {
                unreachable!()
            }
            Rule::EOI => break,
        }
    }

    Ok(Token::SExpression(result))
}

fn parse_list(list: Pair<Rule>) -> Result<Token, Box<dyn std::error::Error>> {
    let mut result: Vec<Token> = Vec::new();

    let rule = list.into_inner();
    let words: Vec<Pair<Rule>> = rule.into_iter().collect();

    for w in words {
        match w.as_rule() {
            Rule::sexpr => result.push(parse_sexpr(w)?),
            Rule::word => result.push(parse_word(w)?),
            Rule::number => result.push(parse_number(w)?),
            Rule::string => result.push(parse_string(w)?),
            Rule::list => result.push(parse_list(w)?),
            Rule::program | Rule::punct | Rule::left_parenthesis | Rule::right_parenthesis => {
                unreachable!()
            }
            Rule::EOI => break,
        }
    }

    Ok(Token::List(result))
}

pub fn parse(s: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut pairs = CoreLangParser::parse(Rule::program, s)?;

    parse_pair(pairs.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::token::{Literal, Token};

    #[test]
    fn parse_example() -> Result<(), Box<dyn std::error::Error>> {
        // This syntax allows empty parentheses
        let token: Vec<Token> = parse("( )")?;
        assert_eq!(token, vec![Token::SExpression(vec![])]);

        // This syntax allows empty parentheses without any spaces
        let token: Vec<Token> = parse("()")?;
        assert_eq!(token, vec![Token::SExpression(vec![])]);

        // It can load Define syntax
        let token: Vec<Token> = parse("( define main 1 )")?;
        assert_eq!(
            token,
            vec![Token::SExpression(vec![
                Token::Literal(Literal::Define),
                Token::Word(String::from("main")),
                Token::Number(1)
            ])]
        );

        // It can load Define syntax
        let token: Vec<Token> = parse("(define main 1)")?;
        assert_eq!(
            token,
            vec![Token::SExpression(vec![
                Token::Literal(Literal::Define),
                Token::Word(String::from("main")),
                Token::Number(1)
            ])]
        );

        Ok(())
    }

    #[test]
    fn parse_multiline() -> Result<(), Box<dyn std::error::Error>> {
        let token: Vec<Token> = parse("( ) ( )")?;
        assert_eq!(
            token,
            vec![Token::SExpression(vec![]), Token::SExpression(vec![])]
        );

        let token: Vec<Token> = parse("( )\n( )")?;
        assert_eq!(
            token,
            vec![Token::SExpression(vec![]), Token::SExpression(vec![])]
        );

        let token: Vec<Token> = parse("() ()")?;
        assert_eq!(
            token,
            vec![Token::SExpression(vec![]), Token::SExpression(vec![])]
        );

        let token: Vec<Token> = parse("()\n()")?;
        assert_eq!(
            token,
            vec![Token::SExpression(vec![]), Token::SExpression(vec![])]
        );

        let token: Vec<Token> = parse("( define main 1 ) ( define main 1 )")?;
        assert_eq!(
            token,
            vec![
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ])
            ]
        );

        let token: Vec<Token> = parse("( define main 1 )\n( define main 1 )")?;
        assert_eq!(
            token,
            vec![
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ])
            ]
        );

        let token: Vec<Token> = parse("(define main 1) (define main 1)")?;
        assert_eq!(
            token,
            vec![
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ])
            ]
        );

        let token: Vec<Token> = parse("(define main 1)\n(define main 1)")?;
        assert_eq!(
            token,
            vec![
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ])
            ]
        );

        Ok(())
    }

    #[test]
    fn parse_multiple_sexpr() -> Result<(), Box<dyn std::error::Error>> {
        let token: Vec<Token> = parse("(define main 1)(define main 1)")?;
        assert_eq!(
            token,
            vec![
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Literal(Literal::Define),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ])
            ]
        );

        Ok(())
    }
}
