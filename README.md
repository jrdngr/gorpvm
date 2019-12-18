# GorpVM

I'm making myself a little pretend computer!

## To do
* ~~Define machine code instructions~~
* Input/output
* Assembly language
* Debugger
* Write some programs
  * Start with some Advent of Code 2019 Intcode 
* Make a LISP & compiler
* Write some more programs 

## Notes

### Operator Abuse
```
// Sorta makes sense
cpu |= [0x01, 0, 0, 0]
cpu &= [0x01, 0, 0, 0]
cpu += [0x01, 0, 0, 0]

// Looks cool
cpu <<= [0x01, 0, 0, 0]

// Other
cpu ^= [0x01, 0, 0, 0]
cpu >>= [0x01, 0, 0, 0]
cpu -= [0x01, 0, 0, 0]
cpu *= [0x01, 0, 0, 0]
cpu /= [0x01, 0, 0, 0]
cpu %= [0x01, 0, 0, 0]
```
