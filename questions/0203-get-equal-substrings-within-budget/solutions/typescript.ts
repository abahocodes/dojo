function equalSubstring(s: string, t: string, maxCost: number): number {
  let left = 0;
  let cost = 0;
  let best = 0;
  for (let right = 0; right < s.length; right++) {
    cost += Math.abs(s.charCodeAt(right) - t.charCodeAt(right));
    while (cost > maxCost) {
      cost -= Math.abs(s.charCodeAt(left) - t.charCodeAt(left));
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}
