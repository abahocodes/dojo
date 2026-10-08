function numRescueBoats(people: number[], limit: number): number {
  const p = [...people].sort((a, b) => a - b);
  let i = 0;
  let j = p.length - 1;
  let boats = 0;
  while (i <= j) {
    if (p[i] + p[j] <= limit) i++;
    j--;
    boats++;
  }
  return boats;
}
