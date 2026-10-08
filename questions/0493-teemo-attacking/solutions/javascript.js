function findPoisonedDuration(timeSeries, duration) {
  let total = 0;
  for (let i = 0; i + 1 < timeSeries.length; i++) {
    // The poison runs its full course unless the next attack resets it.
    total += Math.min(duration, timeSeries[i + 1] - timeSeries[i]);
  }
  return total + duration;
}
