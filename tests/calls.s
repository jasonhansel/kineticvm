	:first -> %LINK
	:third -> %LINK
	1 -> %HALT

first:
	%LINK -> first_save_link
	
	2 -> %OUT
	:second -> %LINK
	:second -> %LINK
	4 -> %OUT

	first_save_link -> %PC

second:
	3 -> %OUT
	%LINK -> %PC

third:
	%LINK -> third_save_link
	:first -> %LINK
	:second -> %LINK
	third_save_link -> %PC