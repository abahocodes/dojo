function carFleet(target: number, position: number[], speed: number[]): number {
  const order = position.map((_, i) => i).sort((a, b) => position[b] - position[a]);
  let fleets = 0;
  let leadDist = 0;
  let leadSpeed = 1;
  for (const i of order) {
    const dist = target - position[i];
    // products stay below 2^53, so the comparison is exact
    if (dist * leadSpeed > leadDist * speed[i]) {
      fleets++;
      leadDist = dist;
      leadSpeed = speed[i];
    }
  }
  return fleets;
}
