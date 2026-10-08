function numDecodings(s) {
  let prev = 1;
  let curr = s[0] !== "0" ? 1 : 0;
  for (let i = 2; i <= s.length; i++) {
    let next = 0;
    if (s[i - 1] !== "0") next += curr;
    const two = Number(s.slice(i - 2, i));
    if (two >= 10 && two <= 26) next += prev;
    prev = curr;
    curr = next;
  }
  return curr;
}
