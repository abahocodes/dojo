function compress(chars) {
  const out = [];
  const n = chars.length;
  let i = 0;
  while (i < n) {
    let j = i;
    while (j < n && chars[j] === chars[i]) j++;
    out.push(chars[i]);
    if (j - i > 1) out.push(String(j - i));
    i = j;
  }
  return out.join("");
}
