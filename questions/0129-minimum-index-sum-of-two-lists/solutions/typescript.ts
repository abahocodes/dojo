function findRestaurant(list1: string[], list2: string[]): string[] {
  const index = new Map<string, number>();
  list1.forEach((s, i) => index.set(s, i));
  let best = Infinity;
  let result: string[] = [];
  for (let j = 0; j < list2.length; j++) {
    const s = list2[j];
    const i = index.get(s);
    if (i === undefined) continue;
    const total = i + j;
    if (total < best) {
      best = total;
      result = [s];
    } else if (total === best) {
      result.push(s);
    }
  }
  return result;
}
