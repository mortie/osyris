(def 'fib (lambda 'x {
	(if [x <= 1]
		{1}
		{(+ (fib [x - 1]) (fib [x - 2]))})
}))

(test-case 'fib {(&&
	[(fib 0) == 1]
	[(fib 1) == 1]
	[(fib 2) == 2]
	[(fib 3) == 3]
	[(fib 4) == 5]
	[(fib 5) == 8]
	[(fib 6) == 13]
	[(fib 15) == 987])
})
