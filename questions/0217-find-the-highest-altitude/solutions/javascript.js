function largestAltitude(gain) {
  let altitude = 0;
  let best = 0;
  for (const g of gain) {
    altitude += g;
    best = Math.max(best, altitude);
  }
  return best;
}
