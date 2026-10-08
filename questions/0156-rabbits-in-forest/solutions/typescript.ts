function numRabbits(answers: number[]): number {
  const counts: number[] = new Array(1000).fill(0);
  for (const x of answers) counts[x]++;
  let total = 0;
  for (let x = 0; x < 1000; x++) {
    if (counts[x] === 0) continue;
    const size = x + 1;
    total += Math.ceil(counts[x] / size) * size;
  }
  return total;
}
