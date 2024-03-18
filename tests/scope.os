(test-case 'scope {
	(def 'x 10)
	(asserteq x 10)

	(def 'get-x-block {x})
	(asserteq (get-x-block) 10)

	(def 'get-x-lambda (lambda {x}))
	(asserteq (get-x-lambda) 10)

	(set 'x 20)
	(asserteq x 20)

	; Lambdas have their own scope, so get-x-lambda remembers the old value
	(asserteq (get-x-lambda) 10)

	; Blocks don't have an associated scope, so get-x-block
	; will be run in the current scope, where x is 20
	(asserteq (get-x-block) 20)
})
