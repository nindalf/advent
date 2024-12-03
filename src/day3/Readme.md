## Advent of Code 2024 [Day 3](https://adventofcode.com/2024/day/3)

\--- Day 3: Mull It Over ---
----------

&quot;Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though,&quot; says the mildly flustered shopkeeper at the [North Pole Toboggan Rental Shop](/2020/day/2). The Historians head out to take a look.

The shopkeeper turns to you. &quot;Any chance you can see why our computers are having issues again?&quot;

The computer appears to be trying to run a program, but its memory (your puzzle input) is *corrupted*. All of the instructions have been jumbled up!

It seems like the goal of the program is just to *multiply some numbers*. It does that with instructions like `mul(X,Y)`, where `X` and `Y` are each 1-3 digit numbers. For instance, `mul(44,46)` multiplies `44` by `46` to get a result of `2024`. Similarly, `mul(123,4)` would multiply `123` by `4`.

However, because the program's memory has been corrupted, there are also many invalid characters that should be *ignored*, even if they look like part of a `mul` instruction. Sequences like `mul(4*`, `mul(6,9!`, `?(12,34)`, or `mul ( 2 , 4 )` do *nothing*.

For example, consider the following section of corrupted memory:

```
xmul(2,4)%&amp;mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
```

Only the four highlighted sections are real `mul` instructions. Adding up the result of each instruction produces `*161*` (`2*4 + 5*5 + 11*8 + 8*5`).

Scan the corrupted memory for uncorrupted `mul` instructions. *What do you get if you add up all of the results of the multiplications?*

To begin, [get your puzzle input](3/input).

Answer:

You can also [Shareon [Bluesky](https://bsky.app/intent/compose?text=%22Mull+It+Over%22+%2D+Day+3+%2D+Advent+of+Code+2024+%23AdventOfCode+https%3A%2F%2Fadventofcode%2Ecom%2F2024%2Fday%2F3) [Twitter](https://twitter.com/intent/tweet?text=%22Mull+It+Over%22+%2D+Day+3+%2D+Advent+of+Code+2024&amp;url=https%3A%2F%2Fadventofcode%2Ecom%2F2024%2Fday%2F3&amp;related=ericwastl&amp;hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.
