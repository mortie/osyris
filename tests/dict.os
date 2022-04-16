(test-case 'dict {(&&
	({
		(def 'd (dict 'a 10 'b 20))
		(&&
			[d.a == 10]
			[d.b == 20]
			[d.whatever == none])
	})

	({
		(def 'd (dict 'x "hello" 'y "goodbye"))
		(mutate 'd dict-set 'z "old" 'z "new")
		(mutate 'd dict-set 'a 10 'b 20)
		(&&
			[d.x == "hello"]
			[d.y == "goodbye"]
			[d.z == "new"]
			[d.a == 10]
			[d.b == 20])
	})
)})
