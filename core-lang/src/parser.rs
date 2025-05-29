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
    Int(i32),
    Data(u64),
}

fn parse_pair(pair: Pair<Rule>) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    match pair.as_rule() {
        Rule::EOI | Rule::punct | Rule::sexpr | Rule::word => unreachable!(),
        Rule::program => {
            let rule = pair.into_inner();
            dbg!(&rule);
            let mut result: Vec<Token> = Vec::new();

            rule.clone().for_each(|w| match w.as_rule() {
                Rule::sexpr => result.push(parse_sexpr(w).unwrap()),
                Rule::word => {
                    let str: String = String::from(w.as_span().as_str());
                    result.push(Token::Word(str));
                }
                Rule::program | Rule::punct => unreachable!(),
                Rule::EOI => return,
            });

            Ok(result)
        }
    }
}

fn parse_sexpr(sexpr: Pair<Rule>) -> Result<Token, Box<dyn std::error::Error>> {
    let mut result: Vec<Token> = Vec::new();
    let mut rule = sexpr.into_inner();
    let mut words: Vec<Pair<Rule>> = rule.into_iter().collect();

    for w in words {
        dbg!(w);
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
        let token: Vec<Token> = parse("( )")?;
        assert_eq!(token, vec![Token::SExpression(vec![])]);

        let token: Vec<Token> = parse("()")?;
        assert_eq!(token, vec![Token::SExpression(vec![])]);

        let token: Vec<Token> = parse("(define main 1)")?;
        assert_eq!(
            token,
            vec![Token::SExpression(vec![
                Token::Word(String::from("define")),
                Token::Word(String::from("main")),
                Token::Int(1)
            ])]
        );

        Ok(())
    }
}
