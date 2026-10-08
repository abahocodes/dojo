def remove_subfolders(folder: list[str]) -> list[str]:
    result = []
    for path in sorted(folder):
        if not result or not path.startswith(result[-1] + "/"):
            result.append(path)
    return result
