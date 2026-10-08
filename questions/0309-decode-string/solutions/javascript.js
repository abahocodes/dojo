function decodeString(s) {
  const stack = [];
  let buf = "";
  let k = 0;
  for (const ch of s) {
    if (ch >= "0" && ch <= "9") {
      k = k * 10 + (ch.charCodeAt(0) - 48);
    } else if (ch === "[") {
      stack.push([buf, k]);
      buf = "";
      k = 0;
    } else if (ch === "]") {
      const [prev, times] = stack.pop();
      buf = prev + buf.repeat(times);
    } else {
      buf += ch;
    }
  }
  return buf;
}
