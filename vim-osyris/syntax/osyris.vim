" Vim syntax file
" Language: Osyris
" Maintainer: Martin Dørum
" Latest Revision: 5 April 2022

if exists("b:current_syntax")
	finish
endif

set iskeyword+=+,-,*,/,=,!,<,>,&,\|,?,@,#,$,%

syn match osyrisFunctionCallee /(\@1<=[^ \t(){}\[\].]\+/
highlight link osyrisFunctionCallee Identifier

"syntax match osyrisIdentifier "[^ \t(){}\[\].]\+"
"highlight link osyrisIdentifier Identifier

syntax keyword osyrisKeyword true false none
highlight link osyrisKeyword Keyword

syntax keyword osyrisFunction def func set mutate if match while print import
syntax keyword osyrisFunction bind lambda lazy do not mod
syntax keyword osyrisFunction list list-push list-pop list-insert list-remove list-map
syntax keyword osyrisFunction list-last list-for list-len
syntax keyword osyrisFunction dict dict-set dict-mutate
syntax keyword osyrisFunction try error
syntax keyword osyrisFunction read write seek
highlight link osyrisFunction Statement

syntax keyword osyrisOperator + - * / == != < <= > >= ?? && \|\|
syntax match osyrisOperator "\."
highlight link osyrisOperator Operator

syntax region osyrisString start=/"/ skip=/\\./ end=/"/
highlight link osyrisString String

syntax match osyrisName "'[^ \t(){}\[\].]\+"
highlight link osyrisName Identifier

syntax match osyrisNumber "-\{,1\}[0-9]\+#[0-9a-fA-F]\+\(\.[0-9a-fA-F]\+\)\{,1\}"
syntax match osyrisNumber "-\{,1}[0-9]\+\(\.[0-9]\+\)\{,1\}"
highlight link osyrisNumber Number

syntax match osyrisComment ";.*$"
highlight link osyrisComment Comment

let b:current_syntax = "osyris"
