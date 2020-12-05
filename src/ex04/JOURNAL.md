# Day 4

## Part 1
I'm having mixed feelings abour Rust today. While creating a [License](https://adventofcode.com/2020/day/4)
struct is easy enough, and having each field being an Option makes it easy to check whether a value
is set or not, I feel like types are getting in the way more than helping. I have to remind myself
that when the types are all correct, and the compiler is happy, the code is _usually_ pretty bug free.
The handful of tests I add are the extra insurance that my logic is sound. The compiler forces me to
code for assumptions that I usually make in Ruby.

At 185 lines of code (and nearly as many lines of test code), this solution got pretty long and verbose.
I had a lot of boiler plate code for setting up each of the types, and optional extracting parts of
strings, or optionally parsing parts of strings into integers. Since a lot of the code has to do with
parsing strings into integers, and confirming they're in a range, there's likely some work that could
be done to refactor this. I may come back and do that at some point as an exercise.

## Big O Notation
O(n)

## What did I learn?
I read once that if you can make the Rust compiler happy, your code will usually run bug free. I would
say that this has been my experience. I'm liking the language so far, and I can tell that the more I
can anticipate compiler warnings, the more I'll like Rust. Thankfully I haven't had to wrestle with the
borrow checker a whole lot yet.

