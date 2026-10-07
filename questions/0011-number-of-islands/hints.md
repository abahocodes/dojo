# Hints

## Hint 1
Think of each land cell as a node in a graph, with edges to its land
neighbours. What are you really counting?

## Hint 2
You're counting connected components. Each time you meet a land cell you
haven't explored yet, you've found a new island. Then you need to "use up"
the rest of that island so you don't count it again.

## Hint 3
Scan every cell. When you hit unvisited land, add one to the count and flood
fill from it (DFS or BFS through the four neighbours), marking every land cell
you reach as visited.
