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

function furthestBuilding(heights: number[], bricks: number, ladders: number): number {
  const ladderClimbs = new MinHeap<number>((x, y) => x < y);
  for (let i = 0; i + 1 < heights.length; i++) {
    const climb = heights[i + 1] - heights[i];
    if (climb <= 0) continue;
    ladderClimbs.push(climb);
    if (ladderClimbs.size > ladders) {
      bricks -= ladderClimbs.pop();
      if (bricks < 0) return i;
    }
  }
  return heights.length - 1;
}
