# Advent of Code 2022

My entries for the annual advent of code puzzles.

## Day 1

Day 1 starts as always with a simple puzzle, in this case summing groups of numbers and finding the highest sum. In the second half, the added complexity is in needing to choose the three highest values.

## Day 2 

Day two steps up the complexity only marginally, introducing a game of rock-paper-scissors. For this one my initial solution works well enough, but I was tempted to try and make a lookup table version once I realized there are only 9 possible input lines so the parta/partb values can be calculated in advance, but unfortunately the performance is much worse due to the resulting string comparison, or hashing if using a hashmap, so I didn't pursue that further.
