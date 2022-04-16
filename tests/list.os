(test-case 'list {(&&
	({
		(def 'l (list 1 2 3))
		(&&
			[l.0 == 1]
			[l.1 == 2]
			[l.2 == 3]
			[l.3 == none])
	})

	({
		(def 'l (list 10 20 30 40))
		(&&
			[l.0 == 10]
			[l.1 == 20]
			[l.2 == 30]
			[l.3 == 40])
	})

	({
		(def 'l (list 5 10 15))
		(mutate 'l list-push 55)
		(&&
			[l.0 == 5]
			[l.1 == 10]
			[l.2 == 15]
			[l.3 == 55])
	})

	({
		(def 'l (list 5 10 15))
		(mutate 'l list-pop)
		(mutate 'l list-push 19)
		(&&
			[l.0 == 5]
			[l.1 == 10]
			[l.2 == 19])
	})
)})
