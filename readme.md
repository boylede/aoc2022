# Advent of Code 2022

My entries for the annual advent of code puzzles.

## Day 1

Day 1 starts as always with a simple puzzle, in this case summing groups of numbers and finding the highest sum. In the second half, the added complexity is in needing to choose the three highest values.

## Day 2 

Day two steps up the complexity only marginally, introducing a game of rock-paper-scissors. For this one my initial solution works well enough, but I was tempted to try and make a lookup table version once I realized there are only 9 possible input lines so the parta/partb values can be calculated in advance, but unfortunately the performance is much worse due to the resulting string comparison, or hashing if using a hashmap, so I didn't pursue that further.

## Day 3 

Day three involves a lot of string comparison, I bet there is a more effecient way to do this but I haven't looked into other solutions yet.

## Day 4

Day four is about checking for overlapping ranges. Hopefully this hints at some future spatial layout themes in this year's puzzles.

## Day 5

Day five brought some interesting towers-of-hanoi elements to the table along with more complicated input parsing. I was really excited by this one. My solution is currently pretty messy, but it does the job for now until I come back to it. I think this one can be parsed and executed in a single pass, so that will be my goal on revisiting. 


## Day 12

Day twelve is a simple pathfinding excersize. I had a lot of trouble with it due to the fact that I assumed all map coordinates were reachable and the part 2 answer had me wondering why my pre-calculating step was never reaching some areas. After rewriting everything a few times I came back and looked at the input to notice that there are areas that can't be walked out of.

