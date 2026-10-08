function corpFlightBookings(bookings: number[][], n: number): number[] {
  const diff: number[] = new Array(n + 1).fill(0);
  for (const [first, last, seats] of bookings) {
    diff[first - 1] += seats;
    diff[last] -= seats;
  }
  const totals: number[] = new Array(n);
  let running = 0;
  for (let i = 0; i < n; i++) {
    running += diff[i];
    totals[i] = running;
  }
  return totals;
}
