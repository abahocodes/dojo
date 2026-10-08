function getSkyline(buildings: number[][]): number[][] {
  // Events [x, -height, right]: starts carry -height, ends carry 0.
  // Sorted by x, then starts before ends, tallest start first.
  const events: [number, number, number][] = [];
  for (const [left, right, height] of buildings) {
    events.push([left, -height, right]);
    events.push([right, 0, 0]);
  }
  events.sort((a, b) => a[0] - b[0] || a[1] - b[1]);

  // Max-heap of [height, right] for buildings that may still stand.
  const live: [number, number][] = [];
  const above = (a: [number, number], b: [number, number]): boolean => a[0] > b[0];
  const push = (item: [number, number]): void => {
    live.push(item);
    let i = live.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!above(live[i], live[p])) break;
      [live[i], live[p]] = [live[p], live[i]];
      i = p;
    }
  };
  const pop = (): void => {
    const last = live.pop()!;
    if (live.length > 0) {
      live[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < live.length && above(live[l], live[m])) m = l;
        if (r < live.length && above(live[r], live[m])) m = r;
        if (m === i) break;
        [live[i], live[m]] = [live[m], live[i]];
        i = m;
      }
    }
  };

  const result: number[][] = [];
  for (const [x, negHeight, right] of events) {
    // Lazily drop buildings that ended at or before x.
    while (live.length > 0 && live[0][1] <= x) pop();
    if (negHeight !== 0) push([-negHeight, right]);
    // The first event at x already settles the height at x.
    const current = live.length > 0 ? live[0][0] : 0;
    if (result.length === 0 || result[result.length - 1][1] !== current) {
      result.push([x, current]);
    }
  }
  return result;
}
