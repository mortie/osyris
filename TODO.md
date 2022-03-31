# TODO

## Make lambdas multi-statement

Currently, lambdas are treaded as function call expressions basically:

```
(def 'print-hello {print "Hello"})
```

It should instead be treated as a proper list of expressions:

```
(def 'print-hello {(print "Hello")})
```

This would make the new `[]` style braces work nicer, by making prefix-style
function calls no longer occupy a privileged space in the syntax.
`{}` in general would be treated more as a list of expressions
rather than a deferred function call expression in itself.

### Refactor eval.rs to focus more on calling multi-

The above change should come with some changes to eval.rs, because it's currently built
around the assumption that `{}`-style braces should mean a prefix-style function call,
not a list of expressions.

### Redesign the match function

Currently, `match` works like this:

```
(match
    {== a b} {print "A is B"}
    {== c d} {print "C is D"})
```

After the above changes, that would require this:

```
(match
    {(== a b)} {(print "A is B")}
    {(== c d)} {(print "C is D")})
```

I think it would be much more natural if it worked like this:

```
(match
    {(== a b) (print "A is B")}
    {(== c d) (print "C is D")})
```

## Add list and dict manipulation functions

All these functions should return a new dict/list with the change applied, but
leave the old value unchanged. Optimization: Mutate if refcount == 1.

* `(list-push list items...)`: Push items to the list
* `(list-pop list)`: Pop an item off the end of the list

* `(dict-set dict {key val}...)`: Set values in the dict
