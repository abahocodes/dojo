function calculateNoParens(s) {
  let total = 0;
  let last = 0;
  let num = 0;
  let op = "+";
  const n = s.length;
  for (let i = 0; i < n; i++) {
    const ch = s[i];
    const isDigit = ch >= "0" && ch <= "9";
    if (isDigit) num = num * 10 + (ch.charCodeAt(0) - 48);
    if ((!isDigit && ch !== " ") || i === n - 1) {
      if (op === "+") {
        total += last;
        last = num;
      } else if (op === "-") {
        total += last;
        last = -num;
      } else if (op === "*") {
        last *= num;
      } else {
        last = Math.trunc(last / num);
      }
      op = ch;
      num = 0;
    }
  }
  return total + last;
}
