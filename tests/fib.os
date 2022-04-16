(def 'fib (lambda 'x {
	(if [x <= 1]
		{1}
		{(+ (fib [x - 1]) (fib [x - 2]))})
}))

(test-case 'fib {
	(assert [(fib 0) == 1])
	(assert [(fib 1) == 1])
	(assert [(fib 2) == 2])
	(assert [(fib 3) == 3])
	(assert [(fib 4) == 5])
	(assert [(fib 5) == 8])
	(assert [(fib 6) == 13])
	(assert [(fib 15) == 987])
})
