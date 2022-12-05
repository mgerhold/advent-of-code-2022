def create_range(range_string: str) -> range:
    start_inclusive, end_inclusive = [int(x) for x in range_string.split('-')]
    return range(start_inclusive, end_inclusive + 1)

def is_fully_overlapping(first: set[int], second: set[int]) -> bool:
    return first.issubset(second) or second.issubset(first)

def is_partially_overlapping(first: set[int], second: set[int]) -> bool:
    return len(first.intersection(second)) > 0

def main() -> None:
    FILENAME = "real_input.txt"

    print("part 1")
    with open(FILENAME) as file:
        print(sum(
            is_fully_overlapping(
                *(set(create_range(x)) for x in line.split(','))
            ) for line in file
        ))

    print("part 2")
    with open(FILENAME) as file:
        print(sum(
            is_partially_overlapping(
                *(set(create_range(x)) for x in line.split(','))
            ) for line in file
        ))

if __name__ == "__main__":
    main()
