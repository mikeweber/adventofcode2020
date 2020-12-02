# Day 1

## Part A
Off to an easy start, per usual. The direct solution to the
[https://adventofcode.com/2020/day/1](expense report problem) is to do two
loops, and compare every value to every other value until you find the two
values that add up to 2020. Though at the time, I was holding a month old
baby and couldn't implement my solution, so all I had was time to think of
a more optimal solution. Since the order of the numbers doesn't matter, I
could have started the inner loop index at 1 + the outer loop index, and
this would have reduced the number of iterations in half, while avoiding
comparing any line to itself, but O(1/2*n^2) is still O(n^2). So I decided
to implement a O(n) solution. While reading the file, I would insert the
values into a HashMap, since HashMap lookup is O(1) time. Then I looped
through each value, and since I knew that the matching value to my current
value was 2020 minus the current value, I checked to see if that matching
value existed in the HashMap. If it did, I'd found my two values.
Otherwise, keep looping.

## Big O notation
O(n)

## What did I learn?
I learned how to read a file in Rust. I found the code online, and will
endeavor to understand more deeply the complicated nested object types
being returned by my new `read_lines` function, but for now it just works.
I will likely move `read_lines` into a util directory, since virtually
every day is going to require reading some input.
