main:
	2 -> %MEM_OBJECT_SIZE
	%MEM_PTR -> first
	2 -> %MEM_OBJECT_SIZE
	%MEM_PTR -> second

	;first points to seocnd
	first -> %MEM_PTR
	1 -> %MEM_OFFSET
	second -> %MEM_VALUE

	; second holds 10
	first -> %MEM_PTR
	1 -> %MEM_OFFSET
	%MEM_VALUE -> %MEM_PTR
	0 -> %MEM_OFFSET
	10 -> %MEM_VALUE

	; first holds 11
	first -> %MEM_PTR
	0 -> %MEM_OFFSET
	11 -> %MEM_VALUE

	; print 11
	first -> %MEM_PTR
	0 -> %MEM_OFFSET
	%MEM_VALUE -> %OUT

	; print 10
	second -> %MEM_PTR
	%MEM_VALUE -> %OUT

	; crash
	; 2 -> %MEM_OFFSET
	; %MEM_VALUE -> %OUT

	1 -> %HALT