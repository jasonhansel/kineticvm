

; make sure types are recorded properly
; mixed, all-ptr, all-int

	24 -> %MEM_OBJECT_SIZE
	%MEM_PTR -> object

	23 -> counter
; Start loop
loop_start:

; Make a new object of size [counter]
	counter -> %MEM_OBJECT_SIZE
	%MEM_PTR -> child_object
; At counter-1 in the new object, store counter
 	counter -> %LHS
	-1 -> %RHS
	child_object -> %MEM_PTR
	%SUM -> %MEM_OFFSET
	counter -> %MEM_VALUE

; write child_object to object[counter]
	object -> %MEM_PTR
	counter -> %MEM_OFFSET
	child_object -> %MEM_VALUE
; Decrement counter
	counter -> %LHS
	-1 -> %RHS
	%SUM -> counter
	:loop_start -> %RHS
; If counter == 0, don't go to start of loop
	counter -> %LHS
	%NOT -> %SKIP_INSTR
	%RHS -> %PC



;;; PART 2: in each child object, at index counter-1, store the object
102 -> %OUT

	23 -> counter




; Start loop
loop_start3:


	object -> %MEM_PTR
	counter -> %MEM_OFFSET
	%MEM_VALUE -> child_object



	; Decrement counter
	counter -> %LHS
	-1 -> %RHS
	%SUM -> newcounter

	child_object -> %MEM_PTR
	newcounter -> %MEM_OFFSET
	counter -> %MEM_VALUE

	newcounter -> %LHS
	-1 -> %RHS
	%SUM -> %MEM_OFFSET
	object -> %MEM_VALUE


; Store new counter
	newcounter -> counter

; If counter == 0, don't go to start of loop
	counter -> %LHS
	-1 -> %RHS
	%SUM -> %LHS
	%NOT -> %SKIP_INSTR
	:loop_start3 -> %PC



;;; Part 3: Get the values from the child objects


103 -> %OUT

	23 -> counter



object -> %MEM_PTR

; Start loop
loop_start2:


	counter -> %MEM_OFFSET
	%MEM_VALUE -> child_object


; Decrement counter
	counter -> %LHS
	-1 -> %RHS
	%SUM -> newcounter


; Print counted value to OUT
	child_object -> %MEM_PTR
	%SUM -> %MEM_OFFSET
	%MEM_OBJECT_SIZE -> %OUT
	%MEM_VALUE -> %OUT

; Decrement counter
	newcounter -> %LHS
	-1 -> %RHS

	%SUM -> %MEM_OFFSET
	%MEM_VALUE -> %MEM_PTR


; Store new counter
	newcounter -> counter

; If counter == 0, don't go to start of loop
	counter -> %LHS
	-1 -> %RHS
	%SUM -> %LHS
	%NOT -> %SKIP_INSTR
	:loop_start2 -> %PC

