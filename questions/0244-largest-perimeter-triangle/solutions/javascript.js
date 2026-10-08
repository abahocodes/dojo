function largestPerimeter(nums) {
  const a = [...nums].sort((x, y) => y - x);
  for (let i = 0; i + 2 < a.length; i++) {
    if (a[i] < a[i + 1] + a[i + 2]) return a[i] + a[i + 1] + a[i + 2];
  }
  return 0;
}
