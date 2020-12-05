# Day 3

## Part 1
Once again, parsing the input of the [tree problem](https://adventofcode.com/2020/day/3)
is the hard part of this problem, and this time was a lot easier. I'm starting to
get a better handle on iterators and parsers. Converting the different characters
into Enums of Open spaces and Tree spaces made iterating over the map trivial.
By looping through each row, and multiplying the row index by the run length, and
finally use modulo of the row length lets each row "loop" infinitely, to get
if the spot is open or a tree. 

## Big O Notation
O(n)
Not really applicable

## What did I learn?
That enums make matching very easy. I've also noticed that I'm fighting less and
less with the compiler, which was my goal for this whole exercise. I still have some way to go, but I'm very happy with my progress.

## Part 2
The way I wrote the code in part 1 worked great for this. The one thing I had to
change was handling when the rise was more than 1.
