So I wrote an intcode assembler using the spec given by day 5 of Advent of Code 2019.

Numbers are literals by default, but can be made addresses by using the `@` prefix. For example,
`133` would be a literal 133, but `@133` would refer to index 133 in the intcode program's memory.

Speaking of which, be careful about your addresses. Depending on how you implement you interpreter,
you may have to pad your intcode program with extra 0s at the end if you intend to have space past
the end of the program to use as something akin to scratch paper. This padding must be done one
literal at a time, unfortunately, since it was much easier for me to parse things that way. If your
interpreter just adds memory when an otherwise out-of-bounds read/write would occur, this is not
something you'll have to worry about.

To use the interpreter, the arguments are fairly similar to the format of `dd`. The input file
argument is in the format `if=/path/to/file` and the output file is in the format
`of=/path/to/file`. So for example, to assemble the `example.aoc` in this repo you might do
`intcode-assembler if=example.aoc of=example.intc`.

So you already know what the opcodes are, but here's a list of the keywords for them that my
assembler recognizes. The base-level bulletpoints are in the format
`KEYWORD (opcode number) - description`.
- ADD - adds two values
    - Takes three parameters:
        - literal/address for the first value
        - literal/address for the second value
        - address to store the result at
- MUL - multiplies two values
    - Takes three parameters:
        - literal/address for the first value
        - literal/address for the second value
        - address to store the result at
- GET - get user input
    - Takes one parameter:
        - address to store input in (note: does not require `@` prefix)
- PRT - print integer
    - Takes one parameter:
        - literal/address for value to print
- JIT - jump if true (non-zero)
    - Takes two parameters:
        - literal/address for value to use as boolean
        - literal/address for value to jump to
- JIF - jump if false (zero)
    - Takes two parameters:
        - literal/address for value to use as boolean
        - literal/address for value to jump to
- SLT - set if less than, sets destination to 1 if the first value is less than the second or 0 if
    it's not
    - Takes three parameters
        - literal/address for the first value
        - literal/address for the second value
        - address to write 1/0 to
- SEQ - set if equal, sets destination to 1 if the first value is equal to the second or 0 if it's
    not
    - Takes three parameters
        - literal/address for the first value
        - literal/address for the second value
        - address to write 1/0 to
- BRK - signifies the end of the intcode program
    - Takes no parameters