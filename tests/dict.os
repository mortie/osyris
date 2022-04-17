(test-case 'dict-basic {
	(def 'd (dict 'a 10 'b 20))

	(asserteq d.a 10)
	(asserteq d.b 20)
	(asserteq d.whatever none)
})

(test-case 'dict-mutate {
	(def 'd (dict 'x "hello" 'y "goodbye"))
	(mutate 'd dict-set 'z "old" 'z "new")
	(mutate 'd dict-set 'a 10 'b 20)

	(asserteq d.x "hello")
	(asserteq d.y "goodbye")
	(asserteq d.z "new")
	(asserteq d.a 10)
	(asserteq d.b 20)
})
