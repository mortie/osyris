use super::ast;

use std::rc::Rc;

pub struct ParseError {
    pub line: i32,
    pub col: i32,
    pub msg: String,
}

pub struct Reader<'a> {
    line: i32,
    col: i32,
    string: &'a [u8],
    idx: usize,
}

impl<'a> Reader<'a> {
    pub fn new(string: &'a [u8]) -> Self {
        Self {
            line: 1,
            col: 1,
            string,
            idx: 0,
        }
    }

    fn peek(&self) -> u8 {
        if self.idx < self.string.len() {
            self.string[self.idx]
        } else {
            0
        }
    }

    fn eof(&self) -> bool {
        return self.idx + 1 == self.string.len();
    }

    fn consume(&mut self) {
        if self.idx < self.string.len() {
            let ch = self.string[self.idx];
            if ch == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }

            self.idx += 1;
        }
    }

    fn err(&self, msg: String) -> ParseError {
        ParseError {
            line: self.line,
            col: self.col,
            msg,
        }
    }
}

fn is_space(ch: u8) -> bool {
    return ch == b' ' || ch == b'\t' || ch == b'\n';
}

fn is_separator(ch: u8) -> bool {
    return is_space(ch)
        || ch == b'('
        || ch == b')'
        || ch == b'{'
        || ch == b'}'
        || ch == b'['
        || ch == b']'
        || ch == b'.';
}

fn skip_space<'a>(r: &mut Reader<'a>) {
    while !r.eof() {
        let ch = r.peek();
        if is_space(ch) {
            r.consume();
        } else if ch == b';' {
            r.consume();
            while !r.eof() {
                if r.peek() == b'\n' {
                    r.consume();
                    break;
                } else {
                    r.consume();
                }
            }
        } else {
            return;
        }
    }
}

fn read_name<'a>(r: &mut Reader<'a>) -> Result<String, ParseError> {
    let start = r.idx;
    while !r.eof() && !is_separator(r.peek()) {
        r.consume();
    }

    if r.idx == start {
        if r.eof() {
            return Err(r.err("Unexpected EOF".to_string()));
        } else {
            return Err(r.err(format!("Unexpected '{}'", r.peek() as char)));
        }
    }

    let s = match std::str::from_utf8(&r.string[start..r.idx]) {
        Ok(s) => s,
        Err(err) => return Err(r.err(format!("Invalid UTF-8: {}", err))),
    };
    Ok(s.to_string())
}

fn parse_string<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    r.consume(); // '"'

    let mut buf: Vec<u8> = Vec::new();
    while !r.eof() {
        let ch = r.peek();
        if ch == b'"' {
            r.consume();
            let s = match String::from_utf8(buf) {
                Ok(s) => s,
                Err(err) => return Err(r.err(format!("Invalid UTF-8: {}", err))),
            };

            return Ok(ast::Expression::String(s));
        } else if ch == b'\\' {
            r.consume();
            if r.eof() {
                return Err(r.err("Unexpected EOF".to_string()));
            }

            let ch = match r.peek() {
                b't' => b'\t',
                b'n' => b'\n',
                b'e' => 0o33,
                b'0' => b'\0',
                b'"' => b'"',
                b'\\' => b'\\',
                ch => return Err(r.err(format!("Invalid escape sequence: \\{}", ch))),
            };

            buf.push(ch);
            r.consume();
        } else {
            buf.push(ch);
            r.consume();
        }
    }

    Err(r.err("Unexpected EOF".to_string()))
}

fn parse_digit(ch: u8, base: u8) -> Result<u8, ()> {
    let num;
    if ch >= b'0' && ch <= b'9' {
        num = ch - b'0';
    } else if ch >= b'a' && ch <= b'z' {
        num = ch - b'a' + 10;
    } else if ch >= b'A' && ch <= b'Z' {
        num = ch - b'A' + 10;
    } else {
        return Err(());
    }

    if num >= base {
        return Err(());
    }

    Ok(num)
}

fn parse_int<'a>(r: &mut Reader<'a>, base: u8) -> (u64, u64) {
    let mut int: u64 = 0;
    let mut div: u64 = 1;
    while !r.eof() {
        let ch = r.peek();
        let digit = match parse_digit(ch, base) {
            Ok(d) => d,
            Err(..) => break,
        };
        div *= base as u64;
        int *= base as u64;
        int += digit as u64;
        r.consume();
    }

    return (int, div);
}

fn parse_number<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    let mut base = 10u8;
    let (mut integral, _) = parse_int(r, 10);
    let mut decimal = 0.0;
    if r.eof() {
        return Ok(ast::Expression::Number(integral as f64));
    }

    if r.peek() == b'#' {
        r.consume();
        if integral > 36 {
            return Err(r.err(format!("Number literal: Max base is 36, got {}", integral)));
        }

        base = integral as u8;
        integral = parse_int(r, base).0;
    }

    if r.peek() == b'.' {
        r.consume();
        let (i, div) = parse_int(r, base);
        decimal = (i as f64) / (div as f64);
    }

    if !r.eof() && !is_separator(r.peek()) {
        return Err(r.err("Invalid number literal".to_string()));
    }

    Ok(ast::Expression::Number(integral as f64 + decimal))
}

fn parse_list<'a>(r: &mut Reader<'a>, closer: u8) -> Result<Vec<ast::Expression>, ParseError> {
    r.consume(); // Opener

    let mut exprs: Vec<ast::Expression> = Vec::new();
    loop {
        skip_space(r);

        if r.peek() == closer {
            r.consume();
            break;
        }

        let expr = match parse(r)? {
            Some(expr) => expr,
            None => return Err(r.err("Unexpected EOF".to_string())),
        };

        exprs.push(expr);
    }

    Ok(exprs)
}

fn parse_infix<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    r.consume(); // '['

    let mut lhs = match parse(r)? {
        Some(expr) => expr,
        None => return Err(r.err("Unexpected EOF".into())),
    };

    loop {
        skip_space(r);

        if r.peek() == b']' {
            r.consume();
            break;
        }

        let infix = match parse(r)? {
            Some(expr) => expr,
            None => return Err(r.err("Unexpected EOF".to_string())),
        };

        let rhs = match parse(r)? {
            Some(expr) => expr,
            None => return Err(r.err("Unexpected EOF".to_string())),
        };

        lhs = ast::Expression::Call(vec![infix, lhs, rhs]);
    }

    Ok(lhs)
}

fn parse_quote<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    r.consume(); // '\''
    if r.peek() == b'(' {
        Ok(ast::Expression::Quote(Rc::new(parse_list(r, b')')?)))
    } else {
        Ok(ast::Expression::String(read_name(r)?))
    }
}

fn parse_braced<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    Ok(ast::Expression::Quote(Rc::new(parse_list(r, b'}')?)))
}

fn parse_call<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    Ok(ast::Expression::Call(parse_list(r, b')')?))
}

fn parse_lookup<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    Ok(ast::Expression::Lookup(read_name(r)?))
}

pub fn parse<'a>(r: &mut Reader<'a>) -> Result<Option<ast::Expression>, ParseError> {
    skip_space(r);

    if r.eof() {
        return Ok(None);
    }

    let ch = r.peek();
    let mut base = if ch == b'"' {
        parse_string(r)?
    } else if ch >= b'0' && ch <= b'9' {
        parse_number(r)?
    } else if ch == b'\'' {
        parse_quote(r)?
    } else if ch == b'(' {
        parse_call(r)?
    } else if ch == b'{' {
        parse_braced(r)?
    } else if ch == b'[' {
        parse_infix(r)?
    } else {
        parse_lookup(r)?
    };

    skip_space(r);
    if r.eof() {
        return Ok(Some(base));
    }

    loop {
        let ch = r.peek();
        if ch == b'.' {
            r.consume();
            skip_space(r);
            let ch = r.peek();
            if ch >= b'0' && ch <= b'9' {
                let num = parse_number(r)?;
                base = ast::Expression::Call(vec![base, num]);
            } else if ch == b'[' {
                let arg = parse_infix(r)?;
                base = ast::Expression::Call(vec![base, arg]);
            } else if ch == b'(' {
                let arg = parse_call(r)?;
                base = ast::Expression::Call(vec![base, arg]);
            } else {
                let name = read_name(r)?;
                base = ast::Expression::Call(vec![base, ast::Expression::String(name)]);
            }
        } else {
            break;
        }
    }

    Ok(Some(base))
}
