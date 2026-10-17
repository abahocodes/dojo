function findItinerary(tickets: string[][]): string[] {
  // Destinations sorted descending, so pop() hands out the smallest one first.
  const graph = new Map<string, string[]>();
  for (const [src, dst] of tickets) {
    if (!graph.has(src)) graph.set(src, []);
    graph.get(src)!.push(dst);
  }
  for (const list of graph.values()) list.sort((a, b) => (a < b ? 1 : a > b ? -1 : 0));

  // Hierholzer: walk greedily; an airport with no tickets left is final,
  // so it goes onto the route (built back to front).
  const route: string[] = [];
  const stack: string[] = ["JFK"];
  while (stack.length) {
    const top = stack[stack.length - 1];
    const out = graph.get(top);
    if (out && out.length) {
      stack.push(out.pop()!);
    } else {
      route.push(stack.pop()!);
    }
  }
  return route.reverse();
}
