# stdlib

* [: print](#-print)
* [: not](#-not)
* [: +](#-)
* [: -](#--)
* [: \*](#--1)
* [: /](#--2)
* [: ==](#--3)
* [: !=](#--4)
* [: <=](#--5)
* [: <](#--6)
* [: >=](#--7)
* [: >](#--8)
* [: ||](#--9)
* [: &&](#--10)
* [: ??](#--11)
* [: def](#-def)
* [: func](#-func)
* [: set](#-set)
* [: mutate](#-mutate)
* [: if](#-if)
* [: match](#-match)
* [: while](#-while)
* [: do](#-do)
* [: bind](#-bind)
* [: with](#-with)
* [: read](#-read)
* [: write](#-write)
* [: seek](#-seek)
* [: error](#-error)
* [: try](#-try)
* [: lazy](#-lazy)
* [: lambda](#-lambda)
* [: list](#-list)
* [: list-push](#-list-push)
* [: list-pop](#-list-pop)
* [: list-map](#-list-map)
* [: list-last](#-list-last)
* [: list-for](#-list-for)
* [: dict](#-dict)
* [: dict-set](#-dict-set)
* [: dict-mutate](#-dict-mutate)

---

### : print

    (print (arg:any)*) -> none

Print the arguments to 'stdout', separated by a space.

---

### : not

    (not val:bool) -> bool

Returns a bool value that's the inverse of its argument.

Examples:

    (not true) -> false
    (not false) -> true

---

### : +

    (+ (val:number)*) -> number

Returns all the numbers added together.

Examples:

    (+ 10 20) -> 30
    (+ 33) -> 33
    (+ 1 2 3 4 5) -> 15
    (+) -> 0

---

### : -

    (- (val:number)*) -> number

Returns all subsequent numbers subtracted from the first number.
If there's only one argument, return the negative of that number.

Examples:

    (- 10) -> -10
    (- 10 3) -> 7
    (- 10 2 3) -> 5
    (-) -> 0

---

### : \*

    (* (val:number)*) -> number

Returns all numbers multiplied by each other.

Examples:

    (* 10) -> 10
    (* 10 3) -> 30
    (* 10 2 3) -> 60
    (*) -> 0

---

### : /

    (/ (val:number)*) -> number

Returns all subsequent numbers divided from the first one.
If there's only one argument, return the reciprocal of that number.

Examples:

    (/ 10) -> 0.1
    (/ 10 2) -> 5
    (/ 30 3 2) -> 5
    (/) -> 0

---

### : ==

    (== (val:any)*) -> bool

Returns true if all values are equal, false otherwise.

Examples:

    (== 10 10) -> true
    (== 20 10) -> false
    (== "Hello" "Hello" "Hello") -> true
    (== "Hello" "Hello" 11) -> false
    (== "11" 11) -> false
    (==) -> true

---

### : !=

    (!= (val:any)*) -> bool

Returns false if all values are equal, true otherwise.

Examples:

    (!= 10 10) -> false
    (!= 20 10) -> true
    (!= "Hello" "Hello" "Hello") -> false
    (!= "Hello" "Hello" 11) -> true
    (!= "11" 11) -> true
    (!=) -> false

---

### : <=

    (<= (val:number)*) -> bool

Returns true if every value is less than or equal to the value to its right.

Examples:

    (<= 10 20 30) -> true
    (<= 10 10 10) -> true
    (<= 4 5) -> true
    (<= 50 40 30) -> false
    (<= 10 20 30 50 40) -> false
    (<= 10) -> true
    (<=) -> true

---

### : <

    (< (val:number)*) -> bool

Returns true if every value is less than the value to its right.

Examples:

    (< 10 20 30) -> true
    (< 10 10 10) -> false
    (< 4 5) -> true
    (< 50 40 30) -> false
    (< 10 20 30 50 40) -> false
    (< 10) -> true
    (<) -> true

---

### : >=

    (>= (val:number)*) -> bool

Returns true if every value is greater than or equal to the value to its right.

Examples:

    (>= 10 20 30) -> false
    (>= 10 10 10) -> true
    (>= 4 5) -> false
    (>= 50 40 30) -> true
    (>= 10 20 30 50 40) -> false
    (>= 10) -> true
    (>=) -> true

---

### : >

    (> (val:number)*) -> bool

Returns true if every value is greater than the value to its right.

Examples:

    (> 10 20 30) -> false
    (> 10 10 10) -> false
    (> 4 5) -> false
    (> 50 40 30) -> true
    (> 10 20 30 50 40) -> false
    (> 10) -> true
    (>) -> true

---

### : ||

    (|| (val:any)*) -> bool

Returns true if any argument is truthy, and false otherwise.
All values other than 'false' and 'none' are considered truthy.

Examples:

    (|| "hello" false) -> true
    (|| false false) -> false
    (|| true) -> true
    (|| true false true) -> true
    (||) -> false

---

### : &&

    (&& (val:any)*) -> bool

Returns false if any argument is falsy, and true otherwise.
The values 'false' and 'none' are considered falsy.

Examples:

    (&& "hello" false) -> false
    (&& false false) -> false
    (&& true) -> true
    (&& true true) -> true
    (&& true false true) -> false
    (&&) -> true

---

### : ??

    (?? (val:any)*) -> bool

Returns the first value that's not 'none'.

Examples:

    (?? none 10 20) -> 10
    (?? none) -> none
    (?? "Hello" none "Goodbye") -> "Hello"
    (?? none none none 3) -> 3
    (??) -> none

---

### : def

    (def (name:string value:any)*) -> none

Defines the given values in the current scope.

Examples:

    (def 'x 10) -> none
    (== x 10) -> true

    (def 'x 40 'y 50) -> none
    (+ x y) -> 90

---

### : func

    (func name:string (arg:string)* body:block) -> none

Defines a lambda with the given name and parameters in the current scope.

Examples:

    (func 'square 'x {
        [x * x]
    })
    (square 10) -> 100
    (square 5) -> 25

    (func 'add 'a 'b {
        [a + b]
    })
    (add 10 20) -> 30
    (add 9 10) -> 19

---

### : set

    (set (name:string value:any)*) -> none

Replace the value with the given name with the given value.

Examples:

    (def 'x 100)
    (== x 100) -> true
    (set 'x 50) -> none
    (== x 50) -> true

    ({
        (set 'x 3)
    })
    (== x 3) -> true

---

### : mutate

    (mutate name:string cb:func (arg:any)*) -> any

Replace the value with the given name with the return value of the callback function.

This:

    (mutate 'x + 1)

Is semantically the same as this:

    (set 'x (+ x 1))

Except that it might allow for refcount==1 optimizations, and that
the modified value is returned.

Examples:

    (def 'x 10)
    (== x 10) -> true
    (mutate 'x + 5) -> 15
    (== x 15) -> true

---

### : if

    (if cond:bool if-body:func (else-body:func)?) -> any

Run the if-body if the condition is truthy, and the else-body
if the condition is falsy. Returns the return value of whichever
function is executed (or none if the condition is false and there's no else-body).

Examples:

    (if [10 == 10] {"10 is 10"} {"10 is not 10"}) -> "10 is 10"
    (if [20 == 10] {"20 is 10"} {"20 is not 10"}) -> "20 is not 10"
    (if true {
        (def 'x 10)
        [x + 30]
    }) -> 40
    (if false {10}) -> none

---

### : match

    (match (case:block)) -> any

Each argument should be a "block", where the first expression is a condition,
and the subsequent expressions form a "body".

Examples:

    (def 'x 10)
    (match
        {[x == 20] "x is 20"}
        {[x == 10] "x is 10"}
    ) -> "x is 10"

    (match
        {false 50}
        {true
            (def 'num 99)
            [num + 1]}
    ) -> 100

---

### : while

    (while condition:func body:func?) -> any

Call the condition function. If it returns true, call the body
if it exists, then loop. If it returns false, return the last thing
the body function returned, or none.

Examples:

    (def 'index 0)
    (def 'sum 1)
    (while {[index < 4]} {
        (set 'sum [sum * 2])
        (set 'index [index + 1])
        sum
    }) -> 16

    (== sum 16) -> true
    (== index 4) -> true

    (while {false}) -> none

---

### : do

    (do (args:any)*) -> any

Returns the last argument. Used to have multiple expressions where one expression
was expected, like the comma operator in C-like languages.

Examples:

    (do 1 2 3) -> 3
    (do (+ 1 3 5) (* 2 4) (- 9 1)) -> 8
    (do) -> none

    ; Expressions may have side-effects, which is generally when you'd need 'do'
    (do (def 'x 10) [x + 5]) -> 15

---

### : bind

    (bind (key:string value:any)* body:func) -> binding

Create a binding. When the binding is called, its body function will be called
with the bound values in its scope.

Examples:

    (def 'f (bind 'x 10 'y 20 {
        [x + y]
    })
    (f) -> 30

    ; A more useful example:
    (func 'create-function {
        (def 'x 10)
        (def 'y 20)
        (bind 'x x 'y y {
            [x + y]
        })
    })
    (def 'f (create-func))
    (f) -> 30

---

### : with

    (with (key:string value:any) body:func) -> any

Call a function with some variables in its scope.

Examples:

    (with 'num [[100 * 3] + [10 * 2]] {
        [num + 5]
    }) -> 325

---

### : read

    (read port:port size:number?) -> any

Read from a port.

---

### : write

    (write port:port value:any) -> none

Write to a port.

---

### : seek

    (seek port:port offset:number from:string?) -> none

Seek a port. 'from' can be:
* set: Seek from the beginning (default)
* end: Seek from the end
* current: Seek from the current position

---

### : error

    (error (message:any)*) -> error

Create an error. An error contains a value:
* If 'error' is called with no arguments, the value is 'none'.
* If 'error' is called with one argument, the value is that argument.
* If 'error' is called with multiple arguments, they are concatenated together
  and the value is the resulting string.

---

### : try

    (try body:func catch:func) -> any

Call 'body'. If it returns an error, call 'catch' with that error's value as an argument.

Examples:

    (try {
        (error "Oh no")
    } (lambda 'err {
        ; somehow handle the error
        "An error occurred"
    })) -> "An error occurred"

---

### : lazy

    (lazy f:func) -> lazy

Create a lazy variable.
A lazy variable contains a reference to a function,
and whenever the variable is used, that function
is implicitly called and the variable evaluates to
the function's return value.

Examples:

    (def 'make-ten {10})
    (def 'ten (lazy make-ten))
    (== ten 10) -> true

---

### : lambda

    (lambda (param:string)* body:block) -> lambda

Create a lambda, which is like a block, but which creates
its own scope when called and which has named arguments.

Examples:

    (def 'add (lambda 'x 'y {
        [x + y]
    }))
    (add 10 20) -> 30
    (add 5 7) -> 12

---

### : list

    (list (value:any)*) -> list

Create a list.

A list can be called with a numeric index as its argument.
The list then returns the value at that index, or 'none'.

Examples:

    ((list) 0) -> none

    (def 'l (list 10 20))
    (l 0) -> 10
    (l 1) -> 20
    (l 2) -> none

    ; This is an alternate function call syntax
    (== l.0 10) -> true
    (== l.1 20) -> true
    (== l.[0 + 1] 20) -> true
    (== l.(+ 0 1) 20) -> true

---

### : list-push

    (list-push l:list (value:any*)) -> list

Returns a new list with new values appended.

Examples:

    (def 'l (list 10))
    (l 0) -> 10
    (l 1) -> none
    ((list-push l 20) 1) -> 20
    (mutate 'l list-push 30 40)
    (l 1) -> 30
    (l 2) -> 40

---

### : list-pop

    (list-pop l:list) -> list

Returns a new list with the last value removed.

Examples:

    (def 'l (list 10 20))
    (l 0) -> 10
    (l 1) -> 20
    (l 2) -> none
    (mutate 'l list-pop)
    (l 0) -> 10
    (l 1) -> none

---

### : list-map

    (list-map l:list transform:func) -> list

Returns a new list where every value is transformed by the transform function.

Examples:

    (def 'l (list 1 2 3))
    (mutate 'l list-map (lambda 'x {[x * 10]}))
    (l 0) -> 10
    (l 1) -> 20
    (l 2) -> 30

---

### : list-last

    (list-last l:list) -> any

Returns the last vaule of a list, or 'none'.

Examples:

    (list-last (list 10 20)) -> 20
    (list-last (list)) -> none

---

### : list-for

    (list-for l:list f:func) -> any

Call the function with every element of the list.
The return value is whatever the last function call returned.

Examples:

    (def 'l (list 1 2 3))
    (def 'sum 0)
    (list-for l (lambda 'el {
        (mutate 'sum + el)
    })) -> 6

---

### : dict

    (dict (key:string value:any)*) -> dict

Create a dict.

A dict can be called with a string key as its argument.
The list then returns the value at that key, or 'none'.

Examples:

    ((dict) 'x) -> none

    (def 'd (dict
        'x 10
        'y 20))
    (d 'x) -> 10
    (d 'y) -> 20
    (d 'z) -> none

    ; This is an alternate function call syntax
    (== d.x 10) -> true
    (== d.y 20) -> true

---

### : dict-set

    (dict-set (key:string value:any)*) -> dict

Returns a new dict with the new keys and values.

Examples:

    (def 'd (dict 'x 10 'y 20))
    (d 'x) -> 10
    (mutate 'd dict-set 'x 30)
    (d 'x) -> 30

---

### : dict-mutate

    (dict-mutate d:dict key:string cb:func (arg:any)*) -> dict

Returns a new dict with the key modified by the callback function.

This:

    (dict-mutate d 'x + 1)

Is semantically the same as this:

    (dict-set d 'x (+ d.x 1))

Except that it might allow for refcount==1 optimizations.

Examples:

    (func 'add-one 'x {
        [x + 1]
    })
    (def 'd (dict 'x 10 'y 20))
    (d 'x) -> 10
    ((dict-mutate d 'x add-one) 'x) -> 11
    ((dict-mutate d 'x + 1) 'x) -> 11

    ; We can use it together with 'mutate'
    (mutate 'd dict-mutate 'x - 3)
    (== d.x 7) -> true
