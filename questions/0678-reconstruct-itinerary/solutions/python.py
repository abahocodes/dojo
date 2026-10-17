from collections import defaultdict


def find_itinerary(tickets: list[list[str]]) -> list[str]:
    # Destinations sorted descending, so pop() hands out the smallest one first.
    graph: dict[str, list[str]] = defaultdict(list)
    for src, dst in sorted(tickets, reverse=True):
        graph[src].append(dst)

    # Hierholzer: walk greedily; an airport with no tickets left is final,
    # so it goes onto the route (built back to front).
    route: list[str] = []
    stack = ["JFK"]
    while stack:
        top = stack[-1]
        if graph[top]:
            stack.append(graph[top].pop())
        else:
            route.append(stack.pop())
    return route[::-1]
