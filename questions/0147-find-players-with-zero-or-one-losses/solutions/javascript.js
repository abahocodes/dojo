function findWinners(matches) {
  let maxId = 0;
  for (const [w, l] of matches) maxId = Math.max(maxId, w, l);
  const losses = new Int32Array(maxId + 1).fill(-1); // -1: never played
  for (const [w, l] of matches) {
    if (losses[w] < 0) losses[w] = 0;
    losses[l] = Math.max(losses[l], 0) + 1;
  }
  const never = [];
  const once = [];
  for (let p = 1; p <= maxId; p++) {
    if (losses[p] === 0) never.push(p);
    else if (losses[p] === 1) once.push(p);
  }
  return [never, once];
}
