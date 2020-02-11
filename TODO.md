# TODO

## Required
- live input (left, right, up, down, enter), erase current screen at each step
- return path upon enter or launch native default apps for the file type
- handle all unwraps / expects
- fix "root", use normal printable item from lib

## Nice to haves:
- horizontal printing with arrows between directory containers?
- condensed print - only show current directory and past x amount, omitting all
  the previous ones
- Remove breadcrumps after horizontal printing is done
- Option to truncate item names to x characters long (which will let more
  directory containers fit horizontally) or to print full names
- Use bold styling for important things (selected directories, directory titles,
  etc)
- Make a color key?

## Questionable:
- Should just the file type indicator be colored or the whole line?
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
