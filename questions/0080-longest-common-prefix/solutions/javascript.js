function longestCommonPrefix(strs) {
  let prefix = strs[0];
  for (let k = 1; k < strs.length && prefix.length > 0; k++) {
    const s = strs[k];
    let i = 0;
    while (i < prefix.length && i < s.length && prefix[i] === s[i]) i++;
    prefix = prefix.slice(0, i);
  }
  return prefix;
}
