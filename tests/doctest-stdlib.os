; This test is auto-generated from doctest.py
; based on: src/stdlib.rs

(test-case 'not {
	(asserteq (not true) false)
	(asserteq (not false) true)
})

(test-case '+ {
	(asserteq (+ 10 20) 30)
	(asserteq (+ 33) 33)
	(asserteq (+ 1 2 3 4 5) 15)
	(asserteq (+) 0)
})

(test-case '- {
	(asserteq (- 10) -10)
	(asserteq (- 10 3) 7)
	(asserteq (- 10 2 3) 5)
	(asserteq (-) 0)
})

(test-case '* {
	(asserteq (* 10) 10)
	(asserteq (* 10 3) 30)
	(asserteq (* 10 2 3) 60)
	(asserteq (*) 0)
})

(test-case '/ {
	(asserteq (/ 10) 0.1)
	(asserteq (/ 10 2) 5)
	(asserteq (/ 30 3 2) 5)
	(asserteq (/) 0)
})

(test-case '== {
	(asserteq (== 10 10) true)
	(asserteq (== 20 10) false)
	(asserteq (== "Hello" "Hello" "Hello") true)
	(asserteq (== "Hello" "Hello" 11) false)
	(asserteq (== "11" 11) false)
	(asserteq (==) true)
})

(test-case '!= {
	(asserteq (!= 10 10) false)
	(asserteq (!= 20 10) true)
	(asserteq (!= "Hello" "Hello" "Hello") false)
	(asserteq (!= "Hello" "Hello" 11) true)
	(asserteq (!= "11" 11) true)
	(asserteq (!=) false)
})

(test-case '<= {
	(asserteq (<= 10 20 30) true)
	(asserteq (<= 10 10 10) true)
	(asserteq (<= 4 5) true)
	(asserteq (<= 50 40 30) false)
	(asserteq (<= 10 20 30 50 40) false)
	(asserteq (<= 10) true)
	(asserteq (<=) true)
})

(test-case '< {
	(asserteq (< 10 20 30) true)
	(asserteq (< 10 10 10) false)
	(asserteq (< 4 5) true)
	(asserteq (< 50 40 30) false)
	(asserteq (< 10 20 30 50 40) false)
	(asserteq (< 10) true)
	(asserteq (<) true)
})

(test-case '>= {
	(asserteq (>= 10 20 30) false)
	(asserteq (>= 10 10 10) true)
	(asserteq (>= 4 5) false)
	(asserteq (>= 50 40 30) true)
	(asserteq (>= 10 20 30 50 40) false)
	(asserteq (>= 10) true)
	(asserteq (>=) true)
})

(test-case '> {
	(asserteq (> 10 20 30) false)
	(asserteq (> 10 10 10) false)
	(asserteq (> 4 5) false)
	(asserteq (> 50 40 30) true)
	(asserteq (> 10 20 30 50 40) false)
	(asserteq (> 10) true)
	(asserteq (>) true)
})

(test-case '|| {
	(asserteq (|| "hello" false) true)
	(asserteq (|| false false) false)
	(asserteq (|| true) true)
	(asserteq (|| true false true) true)
	(asserteq (||) false)
})

(test-case '&& {
	(asserteq (&& "hello" false) false)
	(asserteq (&& false false) false)
	(asserteq (&& true) true)
	(asserteq (&& true true) true)
	(asserteq (&& true false true) false)
	(asserteq (&&) true)
})

(test-case '?? {
	(asserteq (?? none 10 20) 10)
	(asserteq (?? none) none)
	(asserteq (?? "Hello" none "Goodbye") "Hello")
	(asserteq (?? none none none 3) 3)
	(asserteq (??) none)
})

(test-case 'def {
	(def 'x 10)
	(asserteq (+ x 20) 30)

	(def 'x 40 'y 50)
	(asserteq (+ x y) 90)
})

(test-case 'func {
	(func 'square 'x {
		[x * x]
	})
	(asserteq (square 10) 100)
	(asserteq (square 5) 25)

	(func 'add 'a 'b {
		[a + b]
	})
	(asserteq (add 10 20) 30)
	(asserteq (add 9 10) 19)
})
