(test-case 'scope {
	(def 'x 10)
	(asserteq x 10)

	(def 'get-x {x})
	(asserteq (get-x) 10)

	(set 'x 20)
	(asserteq x 20)
	(asserteq (get-x) 10)
})
