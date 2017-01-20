	5 -> factorial_in
	%PC -> factorial_ret
	:factorial -> %PC
	factorial_out -> %OUT

	6 -> factorial_in
	%PC -> factorial_ret
	:factorial -> %PC
	factorial_out -> %OUT

	4 -> factorial_in
	%PC -> factorial_ret
	:factorial -> %PC
	factorial_out -> %OUT

	1 -> %HALT

factorial:
	1 -> factorial_out

loop_start:
	; factorial_out += factorial_in
	factorial_in -> %LHS
	factorial_out -> %RHS
	%PRODUCT -> factorial_out

	; factorial_in -= 1
	factorial_in -> %LHS
	-1 -> %RHS
	%SUM -> factorial_in

	; if factorial_in != 0, restart
	factorial_in -> %LHS
	%NOT -> %SKIP_INSTR
	:loop_start -> %PC

	; return
	factorial_ret -> %LHS
	2 -> %RHS
	%SUM -> %PC