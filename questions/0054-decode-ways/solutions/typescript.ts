function numDecodings(s: string): number {
  let prev = 1;
  let curr = s[0] !== "0" ? 1 : 0;
  for (let i = 2; i <= s.length; i++) {
    let next = 0;
    if (s[i - 1] !== "0") next += curr;
    const pair = Number(s.slice(i - 2, i));
    if (pair >= 10 && pair <= 26) next += prev;
    [prev, curr] = [curr, next];
  }
  return curr;
}
