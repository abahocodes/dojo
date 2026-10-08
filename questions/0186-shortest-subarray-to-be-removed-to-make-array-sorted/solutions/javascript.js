function findLengthOfShortestSubarray(arr) {
  const n = arr.length;
  let right = n - 1;
  while (right > 0 && arr[right - 1] <= arr[right]) right--;
  if (right === 0) return 0;
  let best = right;
  for (let left = 0; left < n; left++) {
    if (left > 0 && arr[left - 1] > arr[left]) break;
    while (right < n && arr[right] < arr[left]) right++;
    best = Math.min(best, right - left - 1);
  }
  return best;
}
