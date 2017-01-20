
	5 -> current
	0 -> total

loop_start: current -> %LHS
	total -> %RHS
	%SUM -> total

	current -> %LHS
	-1 -> %RHS
	%SUM -> current

	:loop_start -> %RHS
	current -> %LHS
	%NOT -> %SKIP_INSTR
	%RHS -> %PC

	; should print 15
	total -> %OUT
