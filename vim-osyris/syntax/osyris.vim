" Vim syntax file
" Language: Osyris
" Maintainer: Martin Dørum
" Latest Revision: 5 April 2022

if exists("b:current_syntax")
	finish
endif

set iskeyword+=+,-,*,/,=,!,<,>,&,\|,?,@,#,$,%

syntax match osyrisIdentifier "[^ \t(){}\[\].]\+"
highlight link osyrisIdentifier Identifier

syntax keyword osyrisKeyword true false none
highlight link osyrisKeyword Keyword

syntax keyword osyrisFunction def set mutate if match while print
syntax keyword osyrisFunction lambda lazy
syntax keyword osyrisFunction list list-push list-pop
syntax keyword osyrisFunction dict dict-set
syntax keyword osyrisFunction try error
highlight link osyrisFunction Statement

syntax keyword osyrisOperator + - * / == != < <= > >= ?? && \|\|
syntax match osyrisOperator "\."
highlight link osyrisOperator Operator

syntax match osyrisString "'[^ \t(){}\[\].]\+"
syntax region osyrisString start=/"/ skip=/\\./ end=/"/
highlight link osyrisString String

syntax match osyrisNumber "-\{,1\}[0-9]\+#[0-9a-fA-F]\+\(\.[0-9a-fA-F]\+\)\{,1\}"
syntax match osyrisNumber "-\{,1}[0-9]\+\(\.[0-9]\+\)\{,1\}"
highlight link osyrisNumber Number

syntax match osyrisComment ";.*$"
highlight link osyrisComment Comment

let b:current_syntax = "osyris"
