(def 'run (lambda 'name {
	(print "Test:" name)
	(import name)
}))

(def 'test-case (lambda 'name 'f {
	(print "\tCase:" name)
	((lambda f))
}))

(def 'assert (lambda 'x {
	(if (not x) {
		(error "Assertion failed")
	})
}))

(def 'asserteq (lambda 'a 'b {
	(if [a != b] {
		(error "Assertion failed: Expected" b "but got" a)
	})
}))

(run "doctest-stdlib.os")
(run "dict.os")
(run "list.os")
(run "fib.os")
(run "scope.os")
