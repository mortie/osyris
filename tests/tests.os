(def 'test-case (lambda 'name 'func {
	(print "Testing" name "...")
	(if (func)
		{(print "\tOK!")}
		{(error "Test failed!")})
}))

(import "dict.os")
(import "list.os")
(import "fib.os")
