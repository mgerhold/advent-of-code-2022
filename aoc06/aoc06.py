def find_solution(line: str, marker_length: int) -> int | None:
    for i in range(marker_length - 1, len(line)):
        if len(set(line[i - marker_length + 1:i + 1])) == marker_length:
            return i + 1
    return None


FILENAME = "real_input.txt"

with open(FILENAME) as file:
    for line in file:
        solution = find_solution(line, 4)
        assert solution is not None
        print(f"part 1: {solution}")

    file.seek(0)

    for line in file:
        solution = find_solution(line, 14)
        assert solution is not None
        print(f"part 2: {solution}")
