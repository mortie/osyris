(def 'fib (lambda 'x {
	(if [x <= 1]
		{1}
		{(+ (fib [x - 1]) (fib [x - 2]))})
}))

(test-case 'fib {
	(asserteq (fib 0) 1)
	(asserteq (fib 1) 1)
	(asserteq (fib 2) 2)
	(asserteq (fib 3) 3)
	(asserteq (fib 4) 5)
	(asserteq (fib 5) 8)
	(asserteq (fib 6) 13)
	(asserteq (fib 15) 987)
})
