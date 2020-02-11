# TODO

## Required
- live input (left, right, up, down, enter), erase current screen at each step
- Use selected state
- return path upon enter
- handle all unwraps / expects
- fix "root", use normal printable item from lib

## Nice to haves:
- print D, F, S, or U next to file (with different colors) or have files themselves be different colors
- horizontal printing with arrows between directory containers?

## Questionable:
- print full path at each step?
- Binary insert for files vs create vec and then sort?

## Cleanup
- refactor long functions
- rustfmt all files
- rename all generic `x`-like variables
- rename bad function names (print_path)
- type annotations on everything
- Remove dead_code tags and silence warnings
- Remove commented out code?
