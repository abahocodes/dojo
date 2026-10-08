function majorityElement(nums) {
  let c1 = 0, c2 = 1, n1 = 0, n2 = 0;
  for (const x of nums) {
    if (x === c1) n1++;
    else if (x === c2) n2++;
    else if (n1 === 0) { c1 = x; n1 = 1; }
    else if (n2 === 0) { c2 = x; n2 = 1; }
    else { n1--; n2--; }
  }
  let f1 = 0, f2 = 0;
  for (const x of nums) {
    if (x === c1) f1++;
    else if (x === c2) f2++;
  }
  const limit = Math.floor(nums.length / 3);
  const result = [];
  if (f1 > limit) result.push(c1);
  if (f2 > limit) result.push(c2);
  return result;
}
