function leastBricks(wall) {
  const seams = new Map();
  let best = 0;
  for (const row of wall) {
    let pos = 0;
    for (let i = 0; i < row.length - 1; i++) {
      pos += row[i];
      const count = (seams.get(pos) || 0) + 1;
      seams.set(pos, count);
      if (count > best) best = count;
    }
  }
  return wall.length - best;
}
