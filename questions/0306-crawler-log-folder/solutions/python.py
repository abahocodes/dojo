def min_operations_to_main(logs: list[str]) -> int:
    depth = 0
    for log in logs:
        if log == "../":
            depth = max(0, depth - 1)
        elif log != "./":
            depth += 1
    return depth
