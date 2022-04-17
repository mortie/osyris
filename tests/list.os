(test-case 'list-basic {
	(def 'l (list 1 2 3))

	(asserteq l.0 1)
	(asserteq l.1 2)
	(asserteq l.2 3)
	(asserteq l.3 none)
})

(test-case 'list-mutate-push {
	(def 'l (list 5 10 15))
	(mutate 'l list-push 55)

	(asserteq l.0 5)
	(asserteq l.1 10)
	(asserteq l.2 15)
	(asserteq l.3 55)
	(asserteq l.4 none)
})

(test-case 'list-mutate-pop-push {
	(def 'l (list 5 10 15))
	(mutate 'l list-pop)
	(mutate 'l list-push 19)

	(asserteq l.0 5)
	(asserteq l.1 10)
	(asserteq l.2 19)
	(asserteq l.3 none)
})

(test-case 'list-map {
	(def 'l (list 1 2 3 4))
	(mutate 'l list-map (lambda 'x {[x * 2]}))

	(asserteq l.0 2)
	(asserteq l.1 4)
	(asserteq l.2 6)
	(asserteq l.3 8)
	(asserteq l.4 none)
})
