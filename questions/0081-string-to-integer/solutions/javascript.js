function stringToInteger(s) {
  const INT_MAX = 2 ** 31 - 1;
  const INT_MIN = -(2 ** 31);
  const n = s.length;
  let i = 0;
  while (i < n && s[i] === " ") i++;
  let sign = 1;
  if (i < n && (s[i] === "+" || s[i] === "-")) {
    if (s[i] === "-") sign = -1;
    i++;
  }
  let value = 0;
  while (i < n && s[i] >= "0" && s[i] <= "9") {
    value = value * 10 + (s.charCodeAt(i) - 48);
    if (value > INT_MAX) return sign === 1 ? INT_MAX : INT_MIN; // stop early: the result is clamped anyway
    i++;
  }
  return sign * value;
}
