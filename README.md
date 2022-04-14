# Osyris

Osyris is a pure Rust programming language with no dependencies.
It's a LISP, and is intended to be easily embeddable.
It's not extremely fast to execute, but it's very fast to parse
and code starts executing in no time.

## API

The main concepts you need to know about when using Osyris as a library are
the Reader, the Scope, and the `eval` function. The Reader parses an input file
into expressions, the Scope is the map from names to variables, and the
eval function takes an expression and a scope and produces a value.

Here's a simple sample program:

```rust
use osyris::{eval, parse, stdlib};
use std::cell::RefCell;
use std::rc::Rc;

// The code we want to execute
static CODE: &'static str = r#"
(def 'fib (lambda 'num {
	(if [num <= 1]
		{1}
		{(+
			(fib [num - 1])
			(fib [num - 2]))})
}))

(print "fib of 20 is" (fib 20))
"#;

fn main() -> Result<(), String> {
    // Create a reader which will give us expressions
    let mut reader = parse::Reader::new(CODE.as_bytes());

    // Create a scope, and populate it with the stdlib
    let scope = Rc::new(RefCell::new(eval::Scope::new()));
    stdlib::init(&scope);

    // Read and eval expressions
    loop {
        // We get a ParseError if the syntax is wrong
        let expr = match parse::parse(&mut reader) {
            Ok(expr) => expr,
            Err(err) => {
                println!("Parse error: {}:{}: {}", err.line, err.col, err.msg);
                break;
            }
        };

        // If the returned expression is None, we reached EOF
        let expr = match expr {
            Some(expr) => expr,
            None => break,
        };

        // Evaluate the expression
        match eval::eval(&expr, &scope) {
            Ok(_value) => (), // Ignore the return value
            Err(err) => {
                println!("Eval error: {}", err);
                break;
            }
        };
    }

    Ok(())
}
```

## Syntax

Like most LISPs, the grammar is extremely simple. There are really only strings, numbers,
identifiers, function calls, and quoted lists ("blocks").

The basics are:

* Number literals: `10`, `20`
* String literals: `"hello world"`, `"with \"escapes\""`, `'identifier-strings`
* Identifiers: `look-like-this`,

Function calls are an opening brace, followed by expressions, followed by a closing brace,
like this: `(print 10 "hello" name)`. The value of the first expression is expected to be
a block (or a built-in function).

Blocks are like function calls but preceded by a quote: `'(print 10 "hello" name)`.
When evaluated, a block becomes a list of expressions. It's common for functions
to take a block as an argument as a kind of callback function. Because they're
so common, Osyris lets you write them with braces instead: `{print 10 "hello" name}`.

## Examples

`if` is a function which takes a condition and one or two blocks:

```osyris
(if (> 10 20)
    {print "10 is greater than 20"}
    {print "10 is not greater than 20"})
```

---

`def` introduces a value in the current scope:

```osyris
(def 'age 32)
(print "Your age is" age)
```

---

You can define functions by defining variables whose bodies are blocks:

```osyris
(def 'say-hello {print "Hello!"})
(say-hello)
```

---

And functions get an implicit `args` value which contains function arguments:

```osyris
(def 'say-hello {print "Hello," (args 0)})
(say-hello "Bob")
```

---

You can use the `bind` function to give arguments names:

```osyris
(def 'say-hello {bind 'name 'age {print "Hello," age "year old" name}})
(say-hello "Annie" 41)
```

## The standard library

These values are populated when you call `stdlib::init`:

* `none`: A variable of type None.
* `(print [values...])`: A function to print stuff to stdout.
* `(+ [numbers...])`: Add together numbers.
* `(- [head] [tail...])`: Subtract the numbers in the tail from the head.
* `(* [numbers...])`: Multiply together numbers.
* `(/ [head] [tail...])`: Divide the number in the head by the numbers in the tail.
* `(== [values...])`: Return 1 if all values are equal, 0 otherwise.
* `(!= [values...])`: Opposite of `==`.
* `(<= [values...])`: Return 1 if each value is smaller than or equal to the next, 0 otherwise.
* `(< [values...])`: Return 1 if each value is smaller than the next, 0 otherwise.
* `(>= [values...])`: Return 1 if each value is greater than or equal to the next, 0 otherwise.
* `(> [values...])`: Return 1 if each value is greater than the next, 0 otherwise.
* `(|| [values...])`: Return the first truthy value, or the last value if all are falsy.
* `(&& [values...])`: Return the first falsy value, or the last value if all are truthy.
* `(def <name> <value>)`: Create a new variable called `name` in the current scope.
* `(set <name> <value>)`: Replace the existing variable called `name`.
* `(if <condition> <body> [else-body])`: Execute `body` if `condition` is truthy,
  otherwise execute `else-body` if it exists.
* `(match [pairs...] [default])`: Execute one out of a set of options,
  based on predicates. Example:
```osyris
(def x 55)
(match {> x 10} {print "It's greater than 10"}
       {< x 10} {print "It's smaller than 10"}
       {print "It's neither greater nor smaller than 10"})
```
* `(while <condition> <body>)`: Execute `body` while `condition` executes to something truthy.
* `(do [values...])`: Return the last value.
* `(bind <array> [names...] <body>)`: Bind values in the `array` to names, then execute `body`.
* `(with [pairs...] <body>)`: Bind values to names, then execute `body`. Example:
```osyris
(with 'x 10
      'y 20
      {print "x + y is" (+ x y)})
```
* `(list [values...])`: Create a list.
* `(dict [pairs...])`: Create a dictionary. Example:
```osyris
(dict 'name "Bob"
      'age 34
      'profession "Programmer")
```
* `(lazy <block>)`: Create a lazy variable.
* `(read <port> [size])`: Read from `port`. If `size` is provided, read a chunk.
* `(write <port> <data>)`: Write `data` to `port`.
* `(seek <port> <offset> [whence])`: Seek to `offset`. By default, `offset` is relative
  to the start, but `whence` can be one of: `'set`, `'end` or `'current` to change that.

## The IO library

These values are populated when you call `iolib::init`.

* `(open <path>)`: Open the file at `path` in read-only mode.
* `(create <path>)`: Create or truncate the file at `path`, open it in write-only mode.
* `(exec <argv>)`: Execute system command.
