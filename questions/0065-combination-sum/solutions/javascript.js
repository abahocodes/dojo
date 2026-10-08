function combinationSum(candidates, target) {
  const sorted = [...candidates].sort((a, b) => a - b);
  const result = [];
  const current = [];

  function backtrack(start, remaining) {
    if (remaining === 0) {
      result.push([...current]);
      return;
    }
    for (let i = start; i < sorted.length; i++) {
      const c = sorted[i];
      if (c > remaining) break; // sorted, so every later candidate is too big as well
      current.push(c);
      backtrack(i, remaining - c); // i, not i + 1: a candidate may be reused
      current.pop();
    }
  }

  backtrack(0, target);
  return result;
}
