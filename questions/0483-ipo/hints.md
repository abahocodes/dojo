# Hints

## Hint 1
Capital never goes down, so a project that is affordable now stays affordable
forever. Among the affordable projects, which one should you do next?

## Hint 2
Always taking the most profitable affordable project is optimal: it raises
your capital the most, which can only unlock more projects. The difficulty is
maintaining "affordable projects" efficiently as capital grows.

## Hint 3
Sort projects by required capital and walk a pointer through them: whenever
capital grows, push every newly affordable project's profit onto a max-heap.
Each round pop the max and add it to capital; stop early when the heap is
empty.
