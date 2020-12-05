# Day 2

## Part 1
I probably made this [valid password problem](https://adventofcode.com/2020/day/2)
more difficult for myself than it needed to be. Here we were tasked with splitting
up a string and turning it into different parts of a rule. I know how to do this
in Ruby, but I'm still getting used to working with iterators being returned by
default, rather than a vec that _can_ be iterated on. Since the implementation of
the easy solution wasn't immediately obvious to me, I decided to go with a full-on
parser. I decided to track which part of the rule I was parsing, then step through
the line, character by character, and switch modes when expected (e.g. when I
reached a non-digit character).

Getting the parser working was the most difficult part. After that, counting the
number of characters in the password that matched the rule was almost trivial.

## Big O notiation
O(n)

## What did I learn?
I learned about the `fold` function for iterators in Rust. For counting the
number of characters that matched, I was originally going to use a filter,
collect the results into a vec, then get the length. Instead I used fold, and
only increased the count when iterating over a "valid" password.

## Part 2
This was pretty easy. Since I already had the rule parsed, I was able to add
a second "valid" method that followed the new criteria.
