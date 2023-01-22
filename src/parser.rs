use super::ast::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{eof, map, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, Expr> {
    terminated(expr, tuple((multispace0, eof)))(input)
}

// expr         = add
// add          = mul ( ( "+" | "-" ) add )?
// mul          = unary ( ( "*" | "/" ) mul )?
// unary        = "-" primary | primary
// primary      = "(" expr ")" | number | identifier
// number       = [0-9]+
// identifier   = [a-zA-Z_] [0-9a-zA-Z_]*

fn expr(s: &str) -> IResult<&str, Expr> {
    add(s)
}

fn add(s: &str) -> IResult<&str, Expr> {
    let (s, head) = mul(s)?;
    let (s, tails) = many0(tuple((
        preceded(multispace0, alt((char('+'), char('-')))),
        mul,
    )))(s)?;
    let expr = tails.into_iter().fold(head, |left, (ch, right)| match ch {
        '+' => Expr::Add {
            left: Box::new(left),
            right: Box::new(right),
        },
        '-' => Expr::Sub {
            left: Box::new(left),
            right: Box::new(right),
        },
        _ => unreachable!(),
    });
    Ok((s, expr))
}

fn mul(s: &str) -> IResult<&str, Expr> {
    let (s, head) = unary(s)?;
    let (s, tails) = many0(tuple((
        preceded(multispace0, alt((char('*'), char('/')))),
        unary,
    )))(s)?;
    let expr = tails.into_iter().fold(head, |left, (ch, right)| match ch {
        '*' => Expr::Mul {
            left: Box::new(left),
            right: Box::new(right),
        },
        '/' => Expr::Div {
            left: Box::new(left),
            right: Box::new(right),
        },
        _ => unreachable!(),
    });
    Ok((s, expr))
}

fn unary(s: &str) -> IResult<&str, Expr> {
    alt((
        map(preceded(tuple((multispace0, char('-'))), primary), |expr| {
            Expr::Minus(Box::new(expr))
        }),
        primary,
    ))(s)
}

fn primary(s: &str) -> IResult<&str, Expr> {
    alt((
        map(
            delimited(
                preceded(multispace0, char('(')),
                expr,
                preceded(multispace0, char(')')),
            ),
            |expr| expr,
        ),
        number,
        identifier,
    ))(s)
}

fn number(s: &str) -> IResult<&str, Expr> {
    map(preceded(multispace0, digit1), |val: &str| {
        Number::I32(val.parse::<i32>().unwrap()).to_expr()
    })(s)
}

fn identifier(s: &str) -> IResult<&str, Expr> {
    map(
        preceded(
            multispace0,
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
        ),
        |ident: &str| Ident::new(ident.to_string()).to_expr_var(),
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error, Err};

    #[test]
    fn parse() {
        assert_eq!(
            add("- 3 + - 2 * - 5"),
            Ok((
                "",
                Expr::Add {
                    left: Box::new(Expr::Minus(Box::new(Number::I32(3).to_expr()))),
                    right: Box::new(Expr::Mul {
                        left: Box::new(Expr::Minus(Box::new(Number::I32(2).to_expr()))),
                        right: Box::new(Expr::Minus(Box::new(Number::I32(5).to_expr())))
                    }),
                }
            ))
        );
        assert_eq!(
            add("3 + - ( 4 + 9 ) * 5"),
            Ok((
                "",
                Expr::Add {
                    left: Box::new(Number::I32(3).to_expr()),
                    right: Box::new(Expr::Mul {
                        left: Box::new(Expr::Minus(Box::new(Expr::Add {
                            left: Box::new(Number::I32(4).to_expr()),
                            right: Box::new(Number::I32(9).to_expr())
                        }))),
                        right: Box::new(Number::I32(5).to_expr())
                    }),
                }
            ))
        );
    }

    #[test]
    fn parse_add() {
        assert_eq!(
            add("3 + 4"),
            Ok((
                "",
                Expr::Add {
                    left: Box::new(Number::I32(3).to_expr()),
                    right: Box::new(Number::I32(4).to_expr())
                }
            ))
        );
        assert_eq!(
            add("3 + 4 + 5"),
            Ok((
                "",
                Expr::Add {
                    left: Box::new(Expr::Add {
                        left: Box::new(Number::I32(3).to_expr()),
                        right: Box::new(Number::I32(4).to_expr())
                    }),
                    right: Box::new(Number::I32(5).to_expr())
                }
            ))
        );
        assert_eq!(
            add("3 - 4"),
            Ok((
                "",
                Expr::Sub {
                    left: Box::new(Number::I32(3).to_expr()),
                    right: Box::new(Number::I32(4).to_expr())
                }
            ))
        );
    }

    #[test]
    fn parse_mul() {
        assert_eq!(
            add("3 * 4"),
            Ok((
                "",
                Expr::Mul {
                    left: Box::new(Number::I32(3).to_expr()),
                    right: Box::new(Number::I32(4).to_expr())
                }
            ))
        );
        assert_eq!(
            add("3 / 4"),
            Ok((
                "",
                Expr::Div {
                    left: Box::new(Number::I32(3).to_expr()),
                    right: Box::new(Number::I32(4).to_expr())
                }
            ))
        );
    }

    #[test]
    fn parse_unary() {
        assert_eq!(
            unary("- 7"),
            Ok(("", Expr::Minus(Box::new(Number::I32(7).to_expr()))))
        );
    }

    #[test]
    fn parse_primary() {
        assert_eq!(primary("( 7 )"), Ok(("", Number::I32(7).to_expr())));
        assert_eq!(primary("( ( 7 ) )"), Ok(("", Number::I32(7).to_expr())));
        assert_eq!(primary("( ( 7 ) )"), Ok(("", Number::I32(7).to_expr())));
    }

    #[test]
    fn parse_number() {
        assert_eq!(number("7"), Ok(("", Number::I32(7).to_expr())));
        assert_eq!(number("23"), Ok(("", Number::I32(23).to_expr())));
        assert_eq!(number("23abc"), Ok(("abc", Number::I32(23).to_expr())));
        assert_eq!(
            number("a7"),
            Err(Err::Error(error::Error::new("a7", error::ErrorKind::Digit)))
        );
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(
            identifier("abc"),
            Ok(("", Ident::new("abc".to_string()).to_expr_var()))
        );
        assert_eq!(
            identifier("_abc012A_BC"),
            Ok(("", Ident::new("_abc012A_BC".to_string()).to_expr_var()))
        )
    }
}
