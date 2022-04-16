(def 'run (lambda 'name {
	(print "Test:" name)
	(import name)
}))

(def 'test-case (lambda 'name 'func {
	(print "\tCase:" name)
	((lambda func))
}))

(def 'assert (lambda 'x {
	(if (not x) {(error "Assertion failed")})
}))

(run "dict.os")
(run "list.os")
(run "fib.os")
