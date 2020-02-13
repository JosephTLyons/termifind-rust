# TODO

## Required
- live input (left, right, up, down, enter), erase current screen at each step
- return path upon enter or launch native default apps for the file type
- handle all unwraps / expects

## Nice to haves:
- horizontal printing with arrows between directory containers?
- Option for vertical print or horizontal print (will be able to print either
  way with new row printing function)
- condensed print - only show current directory and past x amount, omitting all
  the previous ones
- Remove breadcrumps after horizontal printing is done
- Truncation options:
      1. None
      2. Constant: Truncate all to same length
      3. ByFileNameLength: -> 0 -> truncate longest to length
         of 2nd longest file name, 1 -> truncate 1st/2nd to length of 3rd file

         sort file names by length, pick the number to truncate to, and call number 2

      NOTES: If the directory container name is longer than the truncation
      value, then use that as the truncation value
      Number 3: Just past in a number to begin with to determine which level to
      truncate to.  Later, we might be able to use some sort of stats to figure
      out which level to truncate to:
          truncate to the average file name length - preserve the majority of words
          truncate to only the outliers (furthest from the mean crossing some threshold)

      option to truncate directory name too?? prob not, but maybe!

      another options that uses more stats to dynamically choose which directory
      containers to truncate based on average -> try to truncate file names that
      are far away from the average, to the average length

      should truncation length include the ...?

      should ... occur in middle of string? maybe an option for it? if so,
      unit tests

      move text to append into truncation options

- Use bold styling for important things (selected directories, directory titles,
  etc)
- Make a color key?
- Numbered items?
- print by row function using match entirely
- DirecroryContainer, be able to print a fixed height version, use same sort of
  stats that file name truncation will used
- Be able to sort files by name (sort by file name) or by type and then name
  (sort by name with type indicator)

## Questionable:
- Should just the file type indicator be colored or the whole line?
- print full path at each step?
- Binary insert for files vs create vec and then sort?
- Should directory names have their file separator character before them?

## Clean up
- refactor long functions
- rustfmt all files
- rename all generic `x`-like variables
- rename bad function names (print_path)
- type annotations on everything
- Remove dead_code tags and silence warnings
- Remove commented out code?
- Make private and things that don't need to be public
- Make todo into issues and use labels, delete todo
- Put string to append in option
- Sort all pub vs private stuff (struct members, functions, etc.)
- Change print styled file name to be a get ...

## Crates to Consider:
- Crossterm
- ncurses rs
- prettytable

## Research
- Why does the right side of the terminal seem to not be printable to?
