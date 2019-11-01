# DPLL sudoku solver

Solve sudokus using the [DPLL SAT-solving algorithm](https://en.wikipedia.org/wiki/DPLL_algorithm).

This is just a little refresher for my SAT-solving skills :)

## Usage

After building with `cargo build`, provide a sudoku as a string, e.g.:

```sh
$<dpll-solver> "4 83 217
9 5    4
    7
  7  98 6
  37512
2 98  7
    2
 1    4 8
 365 49 7"
 
468392175
975186342
321475689
157249836
683751294
249863751
794628513
512937468
836514927
```

Empty cells can be any character `[^1-9]` and empty cells at the end of a row can be omitted.
