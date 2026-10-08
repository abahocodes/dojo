function topKFrequent(nums: number[], k: number): number[] {
  const counts = new Map<number, number>();
  for (const x of nums) counts.set(x, (counts.get(x) ?? 0) + 1);

  // buckets[f] holds every value that occurs exactly f times
  const buckets: number[][] = Array.from({ length: nums.length + 1 }, () => []);
  for (const [value, freq] of counts) buckets[freq].push(value);

  const result: number[] = [];
  for (let freq = nums.length; freq > 0; freq--) {
    for (const value of buckets[freq]) {
      result.push(value);
      if (result.length === k) return result;
    }
  }
  return result;
}
