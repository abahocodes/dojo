function pushDominoes(dominoes: string): string {
  const n = dominoes.length;
  const res = dominoes.split("");
  // Virtual 'L' before the row and 'R' after it never push anything inward.
  let prevIdx = -1;
  let prev = "L";
  for (let j = 0; j <= n; j++) {
    const cur = j === n ? "R" : dominoes[j];
    if (cur === ".") continue;
    if (prev === cur) {
      for (let k = prevIdx + 1; k < j; k++) res[k] = cur;
    } else if (prev === "R" && cur === "L") {
      let lo = prevIdx + 1;
      let hi = j - 1;
      while (lo < hi) {
        res[lo++] = "R";
        res[hi--] = "L";
      }
    }
    prevIdx = j;
    prev = cur;
  }
  return res.join("");
}
