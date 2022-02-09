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
    return is_space(ch) || ch == b')';
}

fn skip_space<'a>(r: &mut Reader<'a>) {
    while !r.eof() {
        if is_space(r.peek()) {
            r.consume();
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

fn parse_number<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    let mut num: i32 = 0;
    while !r.eof() {
        let ch = r.peek();
        if ch >= b'0' && ch <= b'9' {
            num *= 10;
            num += (ch - b'0') as i32;
            r.consume();
        } else {
            break;
        }
    }

    if !r.eof() && !is_separator(r.peek()) {
        return Err(r.err("Invalid number literal".to_string()));
    }

    Ok(ast::Expression::Number(num))
}

fn parse_parenthesized<'a>(r: &mut Reader<'a>) -> Result<Vec<ast::Expression>, ParseError> {
    r.consume(); // '('

    let mut exprs: Vec<ast::Expression> = Vec::new();
    loop {
        if r.peek() == b')' {
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

fn parse_quote<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    r.consume(); // '\''
    if r.peek() == b'(' {
        Ok(ast::Expression::Quote(Rc::new(parse_parenthesized(r)?)))
    } else {
        Ok(ast::Expression::String(read_name(r)?))
    }
}

fn parse_call<'a>(r: &mut Reader<'a>) -> Result<ast::Expression, ParseError> {
    Ok(ast::Expression::Call(parse_parenthesized(r)?))
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
    if ch == b'"' {
        Ok(Some(parse_string(r)?))
    } else if ch >= b'0' && ch <= b'9' {
        Ok(Some(parse_number(r)?))
    } else if ch == b'\'' {
        Ok(Some(parse_quote(r)?))
    } else if ch == b'(' {
        Ok(Some(parse_call(r)?))
    } else {
        Ok(Some(parse_lookup(r)?))
    }
}
