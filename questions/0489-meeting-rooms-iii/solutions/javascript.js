class Heap {
  constructor(less) {
    this.less = less;
    this.items = [];
  }

  size() {
    return this.items.length;
  }

  peek() {
    return this.items[0];
  }

  push(x) {
    const a = this.items;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.less(a[i], a[p])) break;
      [a[i], a[p]] = [a[p], a[i]];
      i = p;
    }
  }

  pop() {
    const a = this.items;
    const top = a[0];
    const last = a.pop();
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

function mostBooked(n, meetings) {
  const sorted = meetings.slice().sort((a, b) => a[0] - b[0]);
  const free = new Heap((a, b) => a < b); // free room numbers
  for (let r = 0; r < n; r++) free.push(r);
  // Busy rooms as [end time, room], earliest end first, then lowest room.
  const busy = new Heap((a, b) => a[0] < b[0] || (a[0] === b[0] && a[1] < b[1]));
  const count = new Array(n).fill(0);

  for (const [start, end] of sorted) {
    while (busy.size() > 0 && busy.peek()[0] <= start) free.push(busy.pop()[1]);
    let room;
    if (free.size() > 0) {
      room = free.pop();
      busy.push([end, room]);
    } else {
      // Wait for the earliest room; it keeps the meeting's duration.
      const [freeAt, r] = busy.pop();
      room = r;
      busy.push([freeAt + end - start, room]);
    }
    count[room]++;
  }

  let best = 0;
  for (let r = 1; r < n; r++) if (count[r] > count[best]) best = r;
  return best;
}
