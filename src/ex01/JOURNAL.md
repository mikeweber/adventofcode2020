# Day 1

## Part A
Off to an easy start, per usual. The direct solution to the
[expense report problem](https://adventofcode.com/2020/day/1) is to do two
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

## Part B
Ha, I had a feeling this would happen. Now we need to get an inner loop
added, but at least we won't have three loops, so we'll only be bumping
up to O(n^2) and not O(n^3). The crux of my solution still works, but now
we need to add 2 values from the list together, get the difference from the
2020 target value, and check if that difference value is in the list. In
order for the inner loops to work, we now need some sort of sorted list.
Unfortunately the HashMap I used in part 1 won't return an iterated in an
expected order, so if we're trying to use a sublist in the inner loop,
it's impossible to know if the sublist is what we're expecting or not. So
I changed the HashMap to a List of bools, where `true` represents a value
that is in the list. Now we can do a double loop through the list, which
will, by definition, be ordered.

## Big O notation
O(n^2)

## What else did I learn
HashMap iterators won't run in a guaranteed order, so doing nested loops
won't allow for the trick where the inner loop starts 1 index past the
outer loop.
