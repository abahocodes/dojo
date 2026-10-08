function findRestaurant(list1, list2) {
  const index = new Map();
  list1.forEach((s, i) => index.set(s, i));
  let best = Infinity;
  let result = [];
  for (let j = 0; j < list2.length; j++) {
    const s = list2[j];
    if (!index.has(s)) continue;
    const total = index.get(s) + j;
    if (total < best) {
      best = total;
      result = [s];
    } else if (total === best) {
      result.push(s);
    }
  }
  return result;
}
