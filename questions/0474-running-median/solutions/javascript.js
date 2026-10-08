function runningMedian(nums) {
  // Binary heap where before(a, b) means a belongs closer to the top.
  const makeHeap = (before) => {
    const a = [];
    return {
      size: () => a.length,
      peek: () => a[0],
      push(x) {
        a.push(x);
        let i = a.length - 1;
        while (i > 0) {
          const p = (i - 1) >> 1;
          if (!before(a[i], a[p])) break;
          [a[p], a[i]] = [a[i], a[p]];
          i = p;
        }
      },
      pop() {
        const top = a[0];
        const last = a.pop();
        if (a.length > 0) {
          a[0] = last;
          let i = 0;
          for (;;) {
            const l = 2 * i + 1;
            const r = l + 1;
            let m = i;
            if (l < a.length && before(a[l], a[m])) m = l;
            if (r < a.length && before(a[r], a[m])) m = r;
            if (m === i) break;
            [a[m], a[i]] = [a[i], a[m]];
            i = m;
          }
        }
        return top;
      },
    };
  };

  const low = makeHeap((x, y) => x > y); // max-heap: smaller half
  const high = makeHeap((x, y) => x < y); // min-heap: larger half
  const medians = [];
  for (const x of nums) {
    if (low.size() === 0 || x <= low.peek()) low.push(x);
    else high.push(x);
    // Keep low.size() === high.size() or low.size() === high.size() + 1.
    if (low.size() > high.size() + 1) high.push(low.pop());
    else if (high.size() > low.size()) low.push(high.pop());
    medians.push(low.size() > high.size() ? low.peek() : (low.peek() + high.peek()) / 2);
  }
  return medians;
}
