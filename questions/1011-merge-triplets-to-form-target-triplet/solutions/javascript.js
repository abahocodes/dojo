function mergeTriplets(triplets, target) {
  // Merge every triplet that never exceeds the target; the rest are poison.
  const best = [0, 0, 0];
  for (const t of triplets) {
    if (t[0] <= target[0] && t[1] <= target[1] && t[2] <= target[2]) {
      for (let i = 0; i < 3; i++) best[i] = Math.max(best[i], t[i]);
    }
  }
  return best[0] === target[0] && best[1] === target[1] && best[2] === target[2];
}
