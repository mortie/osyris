(test-case 'list-basic {
	(def 'l (list 1 2 3))

	(assert [l.0 == 1])
	(assert [l.1 == 2])
	(assert [l.2 == 3])
	(assert [l.3 == none])
})

(test-case 'list-mutate-push {
	(def 'l (list 5 10 15))
	(mutate 'l list-push 55)

	(assert [l.0 == 5])
	(assert [l.1 == 10])
	(assert [l.2 == 15])
	(assert [l.3 == 55])
	(assert [l.4 == none])
})

(test-case 'list-mutate-pop-push {
	(def 'l (list 5 10 15))
	(mutate 'l list-pop)
	(mutate 'l list-push 19)

	(assert [l.0 == 5])
	(assert [l.1 == 10])
	(assert [l.2 == 19])
	(assert [l.3 == none])
})

(test-case 'list-map {
	(def 'l (list 1 2 3 4))
	(mutate 'l list-map (lambda 'x {[x * 2]}))

	(assert [l.0 == 2])
	(assert [l.1 == 4])
	(assert [l.2 == 6])
	(assert [l.3 == 8])
	(assert [l.4 == none])
})
