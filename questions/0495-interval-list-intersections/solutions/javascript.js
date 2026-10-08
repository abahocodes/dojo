function intervalIntersection(first, second) {
  const result = [];
  let i = 0;
  let j = 0;
  while (i < first.length && j < second.length) {
    const lo = Math.max(first[i][0], second[j][0]);
    const hi = Math.min(first[i][1], second[j][1]);
    if (lo <= hi) result.push([lo, hi]);
    // The interval that ends first cannot meet anything later in the other list.
    if (first[i][1] < second[j][1]) i++;
    else j++;
  }
  return result;
}
