" Vim syntax file
" Language: Osyris
" Maintainer: Martin Dørum
" Latest Revision: 2 April 2022

if exists("b:did_indent")
	finish
endif
let b:did_indent = 1

setlocal indentexpr=OsyrisIndent()

function! OsyrisIndent()
	" Find a non-blank line above the current line.
	let lnum = prevnonblank(v:lnum - 1)

	" Hit the start of the file, use zero indent.
	if lnum == 0
		return 0
	endif

	let ind = indent(lnum)
	let prevline = getline(lnum)
	let currline = getline(v:lnum)

	let opens = len(substitute(prevline, '[^{(\[]', '', 'g'))
	let closes = len(substitute(prevline, '[^})\]]', '', 'g'))
	if opens > closes
		let ind = ind + shiftwidth()
	elseif opens > 0 && closes > opens
		let ind = ind - shiftwidth()
	endif

	let opens = len(substitute(currline, '[^{(\[]', '', 'g'))
	let closes = len(substitute(currline, '[^})\]]', '', 'g'))
	if opens < closes
		let ind = ind - shiftwidth()
	endif

	return ind
endfunction

