function leastInterval(tasks: string[], n: number): number {
    const counts = new Map<string, number>();
    for (const t of tasks) counts.set(t, (counts.get(t) ?? 0) + 1);
    let most = 0;
    for (const c of counts.values()) most = Math.max(most, c);
    let tied = 0;
    for (const c of counts.values()) if (c === most) tied++;
    // most - 1 full rows of width n + 1, then one slot per label tied for the top count
    return Math.max(tasks.length, (most - 1) * (n + 1) + tied);
}
