# Day 5

## Part 1
A [problem of 1s and 0s](https://adventofcode.com/2020/day/4) hiding behind the guise
of letters. I think most people saw thie question for what it was pretty quickly. They
even call it out in the text of the question, but it's not immediately obvious. The
curveball thrown in was trying to make us think that this was two binary numbers that
had to be added together, when in fact, multiplying the first part of the number by 8
is the same as bitshifting it 3 spots over (2^3 = 8). And since the last part of the
ticket code is 3 "bits", I wrote code that treated "B"s and "R"s as 1s, and everything
else as a 0. Granted, the code I wrote doesn't even look at the position of the
character, so I could be parsing invalid ticket codes, but for this exercise, that's
just fine.

## Part 2
To make searching for the missing ticket as effecient as I can think of, I parse all
of the tickets into binary, then sort list (a 1-time performance hit). Then I loop
through each ticket in the list until I find one where the next item in the list isn't
the current ticket number plus 1.

## Big O Notation
O(n^2), or however complex sorting is in a Vec. Everything else is O(n).

## What did I learn?
Rust has a method, u32::from_str_radix, that can convert a string of 1s and 0s into
the equivalent number in u32. My original plan was to build a string and use that
method, but in the end I decided it was just as easy to start with 0, multiply the
current number by 2 for each character, and add 1 when I encountered a "B" or "R".

