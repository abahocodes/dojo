function accountsMerge(accounts: string[][]): string[][] {
  const parent: number[] = accounts.map((_, i) => i);
  const find = (x: number): number => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };

  const owner = new Map<string, number>(); // email -> index of the first account that listed it
  accounts.forEach((account, i) => {
    for (let k = 1; k < account.length; k++) {
      const email = account[k];
      const j = owner.get(email);
      if (j !== undefined) {
        const ri = find(i);
        const rj = find(j);
        if (ri !== rj) parent[ri] = rj;
      } else {
        owner.set(email, i);
      }
    }
  });

  const groups = new Map<number, string[]>();
  for (const [email, i] of owner) {
    const root = find(i);
    const list = groups.get(root);
    if (list) list.push(email);
    else groups.set(root, [email]);
  }

  const cmp = (a: string, b: string): number => (a < b ? -1 : a > b ? 1 : 0);
  const merged: string[][] = [];
  for (const [root, emails] of groups) {
    emails.sort(cmp);
    merged.push([accounts[root][0], ...emails]);
  }
  merged.sort((a, b) => cmp(a[0], b[0]) || cmp(a[1], b[1]));
  return merged;
}
