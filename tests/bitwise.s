    14 -> %LHS ; 01110
    7 -> %RHS  ; 00111
    %BIT_AND -> %OUT ; expect 6
    %BIT_OR -> %OUT ; expect 15
    %BIT_XOR -> %OUT ; expect 9

    8 -> %LHS
    2 -> %RHS
    %SHIFT_L -> %OUT ; expect 32
    %SHIFT_LR -> %OUT ; expect 2
    %SHIFT_AR -> %OUT ; expect 2

    -1 -> %LHS ; 11..1
    2 -> %RHS
    %SHIFT_L -> %OUT ; expect -4
    %SHIFT_LR -> %OUT ; expect 2^15 - 1 = 32767

    %SHIFT_L -> %LHS ; 11...100 = -4
    1 -> %RHS
    %SHIFT_AR -> %OUT ; expect -2


    8 -> %LHS
    %BIT_NOT -> %OUT
    %BIT_NOT -> %LHS
    %BIT_NOT -> %OUT