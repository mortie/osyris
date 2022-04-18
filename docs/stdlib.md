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

### : dict-mutate

    (dict-mutate d:dict key:string cb:func (arg:any)*) -> dict

Create a new dict with the key modified by the callback function.

This:

    (dict-mutate d 'x + 1)

Is semantically the same as this:

    (dict-set d 'x (+ d.x 1))

Except that it might allow for refcount==1 optimizations.
