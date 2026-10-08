function triangleNumber(nums) {
  const a = [...nums].sort((x, y) => x - y);
  let count = 0;
  for (let k = a.length - 1; k >= 2; k--) {
    let i = 0;
    let j = k - 1;
    while (i < j) {
      if (a[i] + a[j] > a[k]) {
        count += j - i;
        j--;
      } else {
        i++;
      }
    }
  }
  return count;
}
