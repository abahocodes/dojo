function validWordAbbreviation(word, abbr) {
  const n = word.length;
  const m = abbr.length;
  let i = 0;
  let j = 0;
  while (i < n && j < m) {
    const c = abbr[j];
    if (c >= "0" && c <= "9") {
      if (c === "0") return false;
      let k = 0;
      while (j < m && abbr[j] >= "0" && abbr[j] <= "9") {
        k = k * 10 + (abbr.charCodeAt(j) - 48);
        j++;
      }
      i += k;
    } else {
      if (word[i] !== c) return false;
      i++;
      j++;
    }
  }
  return i === n && j === m;
}
