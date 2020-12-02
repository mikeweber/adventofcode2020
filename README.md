# Merry Rustmas!

This journaling will be mostly for myself, but I'm posting it to GitHub for posterity.
This will be my 3rd year attempting to do the [Advent of Code](https://adventofcode.com/2020).
I'm hoping to write down my thoughts as I tackle each challenge. This year, the theme will be
Rust. I've used Ruby and Elixir the past two years, so this year I'll be getting in reps with
a newer-to-me language.

## Code layout
With Ruby in 2019, I would write the code in different folders each day. Since it's a scripting
language, it was easy enough to just execute each program on its own. Last year, the Advent of
Code challenges included re-using a lot of code across days, and since Rust requires setting up
a new project just to run the code, plus I actually have to compile code now, I've decided to
write each of the challenges in a single project with a single entrypoint code. So running a
program requires running the same program, but with different arguments. e.g. calling
`cargo run 1a` will run the code for the first part of day 1, `cargo run 2b` will run the code
for the second part of day 2, etc.
