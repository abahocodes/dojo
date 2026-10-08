function countBinarySubstrings(s) {
  let total = 0;
  let prevRun = 0;
  let curRun = 1;
  for (let i = 1; i < s.length; i++) {
    if (s[i] === s[i - 1]) curRun++;
    else {
      total += Math.min(prevRun, curRun);
      prevRun = curRun;
      curRun = 1;
    }
  }
  return total + Math.min(prevRun, curRun);
}
