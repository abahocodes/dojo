function carPooling(trips: number[][], capacity: number): boolean {
  // change[x] = passengers boarding at km x minus passengers leaving at km x
  const change: number[] = new Array(1001).fill(0);
  for (const [passengers, start, end] of trips) {
    change[start] += passengers;
    change[end] -= passengers;
  }
  let load = 0;
  for (const delta of change) {
    load += delta;
    if (load > capacity) return false;
  }
  return true;
}
