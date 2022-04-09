# TODO

## Add list and dict manipulation functions

All these functions should return a new dict/list with the change applied, but
leave the old value unchanged. Optimization: Mutate if refcount == 1.

* `(list-push list items...)`: Push items to the list
* `(list-pop list)`: Pop an item off the end of the list

* `(dict-set dict {key val}...)`: Set values in the dict
