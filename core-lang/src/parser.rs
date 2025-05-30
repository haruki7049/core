use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sexpr.pest"]
pub struct CoreLangParser;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    SExpression(Vec<Token>),
    Word(String),
    Number(u32),
    Data(u64),
}

fn parse_pair(pair: Pair<Rule>) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    match pair.as_rule() {
        Rule::EOI
        | Rule::punct
        | Rule::sexpr
        | Rule::word
        | Rule::number
        | Rule::left_parenthesis
        | Rule::right_parenthesis => unreachable!(),
        Rule::program => {
            let mut result: Vec<Token> = Vec::new();

            let rule = pair.into_inner();
            rule.clone().for_each(|w| match w.as_rule() {
                Rule::sexpr => result.push(parse_sexpr(w).unwrap()),
                Rule::word => {
                    let str: String = String::from(w.as_span().as_str());
                    result.push(Token::Word(str));
                }
                Rule::number => {
                    let number: u32 = w
                        .as_span()
                        .as_str()
                        .parse::<u32>()
                        .expect("PARSE_ERROR: Failed to parse number");
                    result.push(Token::Number(number));
                }
                Rule::program | Rule::punct | Rule::left_parenthesis | Rule::right_parenthesis => {
                    unreachable!()
                }
                Rule::EOI => return,
            });

            Ok(result)
        }
    }
}

fn parse_word(word: Pair<Rule>) -> Result<Token, Box<dyn std::error::Error>> {
    let str: String = String::from(word.as_span().as_str());
    Ok(Token::Word(str))
}

fn parse_number(word: Pair<Rule>) -> Result<Token, Box<dyn std::error::Error>> {
    let number: u32 = word
        .as_span()
        .as_str()
        .parse::<u32>()
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
            Rule::program | Rule::punct | Rule::left_parenthesis | Rule::right_parenthesis => {
                unreachable!()
            }
            Rule::EOI => break,
        }
    }

    Ok(Token::SExpression(result))
}

pub fn parse(s: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut pairs = CoreLangParser::parse(Rule::program, s)?;

    parse_pair(pairs.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::parse;

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
                Token::Word(String::from("define")),
                Token::Word(String::from("main")),
                Token::Number(1)
            ])]
        );

        // It can load Define syntax
        let token: Vec<Token> = parse("(define main 1)")?;
        assert_eq!(
            token,
            vec![Token::SExpression(vec![
                Token::Word(String::from("define")),
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
                    Token::Word(String::from("define")),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Word(String::from("define")),
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
                    Token::Word(String::from("define")),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Word(String::from("define")),
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
                    Token::Word(String::from("define")),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Word(String::from("define")),
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
                    Token::Word(String::from("define")),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Word(String::from("define")),
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
                    Token::Word(String::from("define")),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ]),
                Token::SExpression(vec![
                    Token::Word(String::from("define")),
                    Token::Word(String::from("main")),
                    Token::Number(1)
                ])
            ]
        );

        Ok(())
    }
}
