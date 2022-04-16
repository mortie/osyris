(test-case 'dict-basic {
	(def 'd (dict 'a 10 'b 20))

	(assert [d.a == 10])
	(assert [d.b == 20])
	(assert [d.whatever == none])
})

(test-case 'dict-mutate {
	(def 'd (dict 'x "hello" 'y "goodbye"))
	(mutate 'd dict-set 'z "old" 'z "new")
	(mutate 'd dict-set 'a 10 'b 20)

	(assert [d.x == "hello"])
	(assert [d.y == "goodbye"])
	(assert [d.z == "new"])
	(assert [d.a == 10])
	(assert [d.b == 20])
})
