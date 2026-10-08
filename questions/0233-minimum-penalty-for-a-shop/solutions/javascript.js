function bestClosingTime(customers) {
  let delta = 0;
  let bestDelta = 0;
  let bestHour = 0;
  for (let i = 0; i < customers.length; i++) {
    delta += customers[i] === "Y" ? -1 : 1;
    if (delta < bestDelta) {
      bestDelta = delta;
      bestHour = i + 1;
    }
  }
  return bestHour;
}
