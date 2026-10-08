function findMaxConsecutiveOnes(nums) {
  let best = 0;
  let run = 0;
  for (const x of nums) {
    run = x === 1 ? run + 1 : 0;
    if (run > best) best = run;
  }
  return best;
}
