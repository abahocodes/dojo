function checkValidString(s) {
  // lo..hi is the range of possible counts of unmatched '(' so far.
  let lo = 0;
  let hi = 0;
  for (const ch of s) {
    if (ch === "(") {
      lo++;
      hi++;
    } else if (ch === ")") {
      lo--;
      hi--;
    } else {
      // '*' can be ')', nothing, or '('
      lo--;
      hi++;
    }
    if (hi < 0) return false;
    if (lo < 0) lo = 0;
  }
  return lo === 0;
}
