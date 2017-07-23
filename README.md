**KineticVM** is a bytecode assembler and toy virtual machine for a [transport-triggered architecture](https://en.wikipedia.org/wiki/Transport_triggered_architecture). In other words, it simulates a [register machine](https://en.wikipedia.org/wiki/Register_machine) whose only instruction is "move." All other operations are performed through special *system registers*.

For instance, the following KineticVM assembly code prints the sum of 2 and 3:
```
; Move the constant "2" into the system register LHS
2 -> %LHS
; Move the constant "3" into the system register RHS.
3 -> %RHS
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

## Documentation

Documentation for the full architecture is [available on the wiki](https://github.com/jasonhansel/kineticvm/wiki). More code samples, with corresponding expected output, are contained in the `tests` directory.

The codebase itself is not fully documented at the moment, but `rustdoc` documentation can be generated with `./generate-docs.sh` (currently tested only on Linux).

## Future Plans

### User-facing
* Add more documentation (see above).
* Make the assembler and CLI simpler and more user-friendly (perhaps splitting the assembler and the VM into separate binaries).
    + One issue: assembler can't handle comments at EOF.
* Write more complex tests. This may inspire new features.

### Architecture
* Numerical input
* Support for read-only data (like `.data`)
* Rename (or alter the behavior of) existing registers for simplicity &c. and **make sure** all registers are documented on the wiki
    
### Internals
* Implement a proper garbage collector. Currently, the VM distinguishes between two types (integers and pointers); this feature should allow for [precise collection](https://en.wikipedia.org/wiki/Tracing_garbage_collection#Precise_vs._conservative_and_internal_pointers).
* Create an integrated JIT compiler. It may be useful to have an LLVM backend, like [the one included in HHVM](http://hhvm.com/blog/10205/llvm-code-generation-in-hhvm).

### Future directions
* Write a compiler for a (reasonably) high-level language that targets KineticVM.