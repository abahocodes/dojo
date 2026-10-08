function backspaceCompare(s, t) {
  // Index of the next surviving character at or before i, or -1.
  const prevChar = (text, i) => {
    let skip = 0;
    while (i >= 0) {
      if (text[i] === "#") skip++;
      else if (skip > 0) skip--;
      else return i;
      i--;
    }
    return -1;
  };
  let i = s.length - 1;
  let j = t.length - 1;
  while (true) {
    i = prevChar(s, i);
    j = prevChar(t, j);
    if (i < 0 || j < 0) return i < 0 && j < 0;
    if (s[i] !== t[j]) return false;
    i--;
    j--;
  }
}
