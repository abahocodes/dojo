class MinHeap<T> {
  private a: T[] = [];
  private less: (x: T, y: T) => boolean;
  constructor(less: (x: T, y: T) => boolean) {
    this.less = less;
  }
  get size(): number {
    return this.a.length;
  }
  peek(): T {
    return this.a[0];
  }
  push(x: T): void {
    const a = this.a;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.less(a[i], a[p])) break;
      [a[i], a[p]] = [a[p], a[i]];
      i = p;
    }
  }
  pop(): T {
    const a = this.a;
    const top = a[0];
    const last = a.pop() as T;
    if (a.length > 0) {
      a[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < a.length && this.less(a[l], a[m])) m = l;
        if (r < a.length && this.less(a[r], a[m])) m = r;
        if (m === i) break;
        [a[i], a[m]] = [a[m], a[i]];
        i = m;
      }
    }
    return top;
  }
}

function smallestRange(nums: number[][]): number[] {
  const heap = new MinHeap<[number, number, number]>((x, y) => x[0] < y[0]);
  let high = -Infinity;
  nums.forEach((lst, r) => {
    heap.push([lst[0], r, 0]);
    high = Math.max(high, lst[0]);
  });
  let best = [heap.peek()[0], high];
  for (;;) {
    const [low, r, c] = heap.pop();
    if (high - low < best[1] - best[0]) best = [low, high];
    if (c + 1 === nums[r].length) return best;
    const next = nums[r][c + 1];
    high = Math.max(high, next);
    heap.push([next, r, c + 1]);
  }
}
