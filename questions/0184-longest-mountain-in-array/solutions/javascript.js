function longestMountain(arr) {
  let best = 0;
  let up = 0;
  let down = 0;
  for (let i = 1; i < arr.length; i++) {
    if (arr[i - 1] === arr[i] || (down > 0 && arr[i - 1] < arr[i])) {
      up = 0;
      down = 0;
    }
    if (arr[i - 1] < arr[i]) up++;
    else if (arr[i - 1] > arr[i]) down++;
    if (up > 0 && down > 0) best = Math.max(best, up + down + 1);
  }
  return best;
}
