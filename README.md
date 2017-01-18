**KineticVM** is a bytecode assembler and virtual machine for a [transport-triggered architecture](https://en.wikipedia.org/wiki/Transport_triggered_architecture). In other words, it simulates a [register machine](https://en.wikipedia.org/wiki/Register_machine) whose only instruction is "move." All other operations are performed through special *system registers*.

For instance, the following KineticVM assembly code prints the sum of 2 and 3:
```
; Move the constant "2" into the system register LHS
$2 -> %LHS
; Move the constant "3" into the system register RHS.
$3 -> %RHS
; The value of the SUM register is always equal to the sum of LHS and RHS.
; Here, therefore, it contains the number 5. 
; Moving a value into the system register OUT prints it to the screen.
; So moving SUM to OUT will print the number 5:
%SUM -> %OUT
```

If this code is saved in `test.s`, one can assemble and execute it with the following commands:
```
$ cargo build
$ ./target/debug/kineticvm assemble test.s > test.o
$ ./target/debug/kineticvm execute test.o
5
```

Further documentation is forthcoming. In the meantime, more code samples, with corresponding expected output, are contained in the `tests` directory.