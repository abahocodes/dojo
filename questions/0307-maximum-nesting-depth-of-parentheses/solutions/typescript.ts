function maxDepthParens(s: string): number {
  let depth = 0;
  let best = 0;
  for (const ch of s) {
    if (ch === "(") best = Math.max(best, ++depth);
    else if (ch === ")") depth--;
  }
  return best;
}
