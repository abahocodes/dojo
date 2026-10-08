function findMinArrowShots(points: number[][]): number {
  // Greedy: shoot each arrow at the right end of the balloon that ends first.
  const ordered = [...points].sort((a, b) => a[1] - b[1]);
  let arrows = 1;
  let pos = ordered[0][1];
  for (const [start, end] of ordered) {
    if (start > pos) {
      arrows++;
      pos = end;
    }
  }
  return arrows;
}
