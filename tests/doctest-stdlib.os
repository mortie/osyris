; This test is auto-generated from doctest.py
; based on: src/stdlib.rs

(test-case 'not {
	(asserteq (not true) false)
	(asserteq (not false) true)
})

(test-case 'mod {
	(asserteq (mod 11 3) 2)
	(asserteq [12 mod 3] 0)
	(asserteq (mod 9 2) 1)
	(asserteq (mod 8 2) 0)
})

(test-case '+ {
	(asserteq (+ 10 20) 30)
	(asserteq (+ 33) 33)
	(asserteq [10 + 30] 40)
	(asserteq (+ 1 2 3 4 5) 15)
	(asserteq (+) 0)
})

(test-case '- {
	(asserteq (- 10) -10)
	(asserteq (- 10 3) 7)
	(asserteq [10 - 4] 6)
	(asserteq (- 10 2 3) 5)
	(asserteq (-) 0)
})

(test-case '* {
	(asserteq (* 10) 10)
	(asserteq [10 * 5] 50)
	(asserteq (* 10 3) 30)
	(asserteq (* 10 2 3) 60)
	(asserteq (*) 0)
})

(test-case '/ {
	(asserteq (/ 10) 0.1)
	(asserteq (/ 10 2) 5)
	(asserteq (/ 30 3 2) 5)
	(asserteq [200 / 10] 20)
	(asserteq (/) 0)
})

(test-case '== {
	(asserteq (== 10 10) true)
	(asserteq (== 20 10) false)
	(asserteq (== "Hello" "Hello" "Hello") true)
	(asserteq (== "Hello" "Hello" 11) false)
	(asserteq (== "11" 11) false)
	(asserteq (==) true)
	(asserteq (== (list 1 2 3) (list 1 2 3)) true)
	(asserteq (==
		(list (list (list 1) (list 2)))
		(list (list (list 1) (list 2)))) true)
	(asserteq (== (list 1 2 3) (list 1 2 4)) false)
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
	(asserteq (def 'x 10) none)
	(asserteq (== x 10) true)
	(asserteq (def 'x 40 'y 50) none)
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

(test-case 'set {
	(def 'x 100)
	(asserteq x 100)
	(asserteq (set 'x 50) none)
	(asserteq x 50)
	({
		(set 'x 3)
	})
	(asserteq x 3)
})

(test-case 'mutate {
	(def 'x 10)
	(asserteq x 10)
	(asserteq (mutate 'x + 5) 15)
	(asserteq x 15)
})

(test-case 'if {
	(asserteq (if [10 == 10] {"10 is 10"} {"10 is not 10"}) "10 is 10")
	(asserteq (if [20 == 10] {"20 is 10"} {"20 is not 10"}) "20 is not 10")
	(asserteq (if true {
		(def 'x 10)
		[x + 30]
	}) 40)
	(asserteq (if false {10}) none)
})

(test-case 'match {
	(def 'x 10)
	(asserteq (match
		{[x == 20] "x is 20"}
		{[x == 10] "x is 10"}
	) "x is 10")
	(asserteq (match
		{false 50}
		{true
			(def 'num 99)
			[num + 1]}
	) 100)
})

(test-case 'while {
	(def 'index 0)
	(def 'sum 1)
	(asserteq (while {[index < 4]} {
		(set 'sum [sum * 2])
		(set 'index [index + 1])
		sum
	}) 16)
	(asserteq sum 16)
	(asserteq index 4)
	(asserteq (while {false}) none)
})

(test-case 'do {
	(asserteq (do 1 2 3) 3)
	(asserteq (do (+ 1 3 5) (* 2 4) (- 9 1)) 8)
	(asserteq (do) none)
	(asserteq (do (def 'x 10) [x + 5]) 15)
})

(test-case 'bind {
	(def 'f (bind 'x 10 'y 20 {
		[x + y]
	}))
	(asserteq (f) 30)
	(func 'create-function {
		(def 'x 10)
		(def 'y 20)
		(bind 'x x 'y y {
			[x + y]
		})
	})
	(def 'f (create-function))
	(asserteq (f) 30)
})

(test-case 'with {
	(asserteq (with 'num [[100 * 3] + [10 * 2]] {
		[num + 5]
	}) 325)
})

(test-case 'try {
	(asserteq (try {
		(error "Oh no")
	} (lambda 'err {
		; somehow handle the error
		"An error occurred"
	})) "An error occurred")
})

(test-case 'lazy {
	(def 'make-ten {10})
	(def 'ten (lazy make-ten))
	(asserteq ten 10)
})

(test-case 'lambda {
	(def 'add (lambda 'x 'y {
		[x + y]
	}))
	(asserteq (add 10 20) 30)
	(asserteq (add 5 7) 12)
	(asserteq [9 add 10] 19)
})

(test-case 'list {
	(asserteq ((list) 0) none)
	(def 'l (list 10 20))
	(asserteq (l 0) 10)
	(asserteq (l 1) 20)
	(asserteq (l 2) none)
	(asserteq l.0 10)
	(asserteq l.1 20)
	(asserteq l.[0 + 1] 20)
	(asserteq l.(+ 0 1) 20)
})

(test-case 'list-push {
	(def 'l (list 10))
	(asserteq l.0 10)
	(asserteq l.1 none)
	(asserteq ((list-push l 20) 1) 20)
	(mutate 'l list-push 30 40)
	(asserteq l.1 30)
	(asserteq l.2 40)
})

(test-case 'list-pop {
	(def 'l (list 10 20))
	(asserteq l.0 10)
	(asserteq l.1 20)
	(asserteq l.2 none)
	(mutate 'l list-pop)
	(asserteq l.0 10)
	(asserteq l.1 none)
})

(test-case 'list-insert {
	(def 'l (list 1 2 3))
	(mutate 'l list-insert 0 10)
	(asserteq l.0 10)
	(asserteq l.1 1)
	(asserteq l.2 2)
	(mutate 'l list-insert 2 99 100)
	(asserteq l.1 1)
	(asserteq l.2 99)
	(asserteq l.3 100)
	(asserteq l.4 2)
})

(test-case 'list-remove {
	(def 'l (list 1 2 3))
	(mutate 'l list-remove 1)
	(asserteq l.0 1)
	(asserteq l.1 3)
	(asserteq l.3 none)
	(def 'l (list 1 2 3 4))
	(mutate 'l list-remove 1 3)
	(asserteq l.0 1)
	(asserteq l.1 4)
	(asserteq l.2 none)
})

(test-case 'list-map {
	(def 'l (list 1 2 3))
	(mutate 'l list-map (lambda 'x {[x * 10]}))
	(asserteq l.0 10)
	(asserteq l.1 20)
	(asserteq l.2 30)
})

(test-case 'list-last {
	(asserteq (list-last (list 10 20)) 20)
	(asserteq (list-last (list)) none)
})

(test-case 'list-for {
	(def 'l (list 1 2 3))
	(def 'sum 0)
	(asserteq (list-for l (lambda 'el {
		(mutate 'sum + el)
	})) 6)
})

(test-case 'dict {
	(asserteq ((dict) 'x) none)
	(def 'd (dict
		'x 10
		'y 20))
	(asserteq d.x 10)
	(asserteq d.y 20)
	(asserteq d.z none)
	(asserteq d.x 10)
	(asserteq d.y 20)
})

(test-case 'dict-set {
	(def 'd (dict 'x 10 'y 20))
	(asserteq d.x 10)
	(mutate 'd dict-set 'x 30)
	(asserteq d.x 30)
})

(test-case 'dict-mutate {
	(func 'add-one 'x {
		[x + 1]
	})
	(def 'd (dict 'x 10 'y 20))
	(asserteq d.x 10)
	(asserteq ((dict-mutate d 'x add-one) 'x) 11)
	(asserteq ((dict-mutate d 'x + 1) 'x) 11)
	(mutate 'd dict-mutate 'x - 3)
	(asserteq d.x 7)
})
