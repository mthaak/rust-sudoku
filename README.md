# Rust Sudoku Solver

This a sudoku solver written in Rust. It uses Knuth's Algorithm X to solve the puzzle. The idea and algorithm I got
from [demofox](https://blog.demofox.org/2022/10/30/rapidly-solving-sudoku-n-queens-pentomino-placement-and-more-with-knuths-algorithm-x-and-dancing-links/).

Because Algorithm X uses backtracking, it is very fast and can solve even the hardest puzzles (like AI Escargot) in a
fraction of a second.

To test my Algorithm X implementation, I also applied it to the Wikipedia exact
cover [basic example](https://en.wikipedia.org/wiki/Exact_cover#Detailed_example) and
the [n queens problem](https://en.wikipedia.org/wiki/Eight_queens_puzzle). This could be done by converting the problems
to the exact cover problem and then solving it with Algorithm X.
