from collections import defaultdict
import math
from queue import PriorityQueue


class Map():
    def __init__(self, filename: str) -> None:
        with open(filename) as file:
            lines = file.readlines()
        self._width = len(lines[0].strip())
        self._contents = "".join(line.strip() for line in lines)
        assert len(self._contents) % self._width == 0, "invalid contents length"

    def width(self) -> int:
        return self._width

    def height(self) -> int:
        return len(self._contents) // self._width

    def _coords_to_index(self, coords: tuple[int, int]) -> int:
        x, y = coords
        return y * self.width() + x

    def _index_to_coords(self, index: int) -> tuple[int, int]:
        return (index % self.width(), index // self.width())

    @staticmethod
    def _char_to_height(char: str) -> int:
        assert len(char) == 1, "expected single character"
        assert (ord(char) >= ord("a") and ord(char) <= ord("z")
                ) or char == "S" or char == "E", "invalid char"
        if char == "S":
            height_symbol = "a"
        elif char == "E":
            height_symbol = "z"
        else:
            height_symbol = char
        return ord(height_symbol) - ord("a")

    def get(self, coords: tuple[int, int]) -> int:
        return self._char_to_height(self._contents[self._coords_to_index(coords)])

    def _find(self, char: str) -> tuple[int, int]:
        assert len(char) == 1, "expected single character"
        return self._index_to_coords(self._contents.index(char))

    def find_start(self) -> tuple[int, int]:
        return self._find("S")

    def find_end(self) -> tuple[int, int]:
        return self._find("E")

    def get_neighbors(self, coords: tuple[int, int]) -> list[tuple[int, int]]:
        x0, y0 = coords
        height = self.get(coords)
        result: list[tuple[int, int]] = list()

        possible_neighbors = [(x0 - 1, y0), (x0 + 1, y0),
                              (x0, y0 - 1), (x0, y0 + 1)]
        for possible_neighbor in possible_neighbors:
            x, y = possible_neighbor
            if x < 0 or x >= self.width() or y < 0 or y >= self.height():
                continue
            if self.get(possible_neighbor) <= height + 1:
                result.append(possible_neighbor)
        return result

    def __str__(self) -> str:
        result = ""
        for y in range(self.height()):
            result += self._contents[y * self.width():(y + 1) * self.width()]
            result += "\n"
        return result


def find_best_path(
    map: Map,
    start: None | tuple[int, int] = None
) -> list[tuple[int, int]]:
    class Node():
        def __init__(self, coords: tuple[int, int], costs: float) -> None:
            assert isinstance(costs, float)
            self.coords = coords
            self.costs = costs

        def __lt__(self, other: "Node") -> bool:
            return self.costs < other.costs

        def __gt__(self, other: "Node") -> bool:
            return self.costs > other.costs

        def __eq__(self, other: object) -> bool:
            if not isinstance(other, Node):
                return False
            return self.coords == other.coords

    class Costs():
        def __init__(self, value: None | float = None) -> None:
            assert value is None or isinstance(value, float)
            self._value = value

        def is_infinite(self) -> bool:
            return self._value is None

        def value(self) -> float:
            assert not self.is_infinite()
            assert isinstance(self._value, float)
            return self._value

        def __lt__(self, other: "Costs") -> bool:
            if self.is_infinite():
                return False
            if other.is_infinite():
                return True
            assert isinstance(self._value, float) and isinstance(
                other._value, float)
            return self._value < other._value

        def __gt__(self, other: "Costs") -> bool:
            if self.is_infinite() and not other.is_infinite():
                return True
            if self.is_infinite():
                assert other.is_infinite()
                return False
            assert not self.is_infinite()
            if other.is_infinite():
                return False
            assert not other.is_infinite()
            assert isinstance(self._value, float) and isinstance(
                other._value, float)
            return self._value < other._value

    def heuristic(start: tuple[int, int], end: tuple[int, int]) -> float:
        x1, y1 = start
        x2, y2 = end
        return math.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)

    def backtrace_path(
        predecessors: dict[tuple[int, int], tuple[int, int]],
        end: tuple[int, int]
    ) -> list[tuple[int, int]]:
        result: list[tuple[int, int]] = list()
        current = end
        while current in predecessors:
            predecessor = predecessors[current]
            result.append(predecessor)
            current = predecessor
        result.reverse()
        return result

    if start is None:
        start = map.find_start()
    end = map.find_end()
    to_visit: PriorityQueue[Node] = PriorityQueue()
    to_visit.put(Node(start, 0.0))

    predecessors: dict[tuple[int, int], tuple[int, int]] = dict()

    costs: defaultdict[tuple[int, int], Costs] = defaultdict(Costs)
    costs[start] = Costs(0.0)

    # estimated_costs: defaultdict[tuple[int, int], Costs] = defaultdict(Costs)
    # estimated_costs[start] = Costs(heuristic(start, end))

    while not to_visit.empty() > 0:
        current = to_visit.get()
        if current.coords == end:
            # path found
            return backtrace_path(predecessors, end)

        neighbors = map.get_neighbors(current.coords)
        for neighbor in neighbors:
            # every move costs 1 step
            preliminary_costs = Costs(costs[current.coords].value() + 1)
            if preliminary_costs < costs[neighbor]:
                # this path is better than any other leading to this position we have found before
                predecessors[neighbor] = current.coords
                costs[neighbor] = preliminary_costs
                if neighbor not in (entry.coords for entry in to_visit.queue):
                    to_visit.put(
                        Node(neighbor, preliminary_costs.value() + heuristic(neighbor, end)))

    # path not found
    return []


def print_path(path: list[tuple[int, int]], width: int, height: int, end: tuple[int, int]) -> None:
    rows: list[str] = ["".join(["." for _ in range(width)])
                       for _ in range(height)]
    for i in range(len(path) - 1):
        x1, y1 = path[i]
        x2, y2 = path[i + 1]
        if x2 > x1:
            symbol = ">"
        elif x2 < x1:
            symbol = "<"
        elif y2 > y1:
            symbol = "v"
        else:
            symbol = "^"
        row = rows[y1]
        row = row[:x1] + symbol + row[x1 + 1:]
        rows[y1] = row

    pred_x, pred_y = path[-1]
    end_x, end_y = end
    if end_x > pred_x:
        symbol = ">"
    elif end_x < pred_x:
        symbol = "<"
    elif end_y > pred_y:
        symbol = "v"
    else:
        symbol = "^"
    pred_row = rows[pred_y]
    pred_row = pred_row[:pred_x] + symbol + pred_row[pred_x + 1:]
    rows[pred_y] = pred_row

    end_row = rows[end_y]
    end_row = end_row[:end_x] + "E" + end_row[end_x + 1:]
    rows[end_y] = end_row

    for row in rows:
        print(row)


def main() -> None:
    map = Map("real_input.txt")
    print(map)
    result = find_best_path(map)
    print_path(result, map.width(), map.height(), map.find_end())
    print(f"\n{len(result)} steps")

    # part 2 is slow as hell, but it works ¯\_(ツ)_/¯
    min_length: None | int = None
    for x in range(map.width()):
        for y in range(map.height()):
            start = (x, y)
            if map.get(start) != 0:
                # not the lowest elevation
                continue
            path = find_best_path(map, start)
            if len(path) == 0:
                # no path found from here
                continue
            length = len(path)
            if min_length is None or length < min_length:
                min_length = length

        # show progress because it's so slow
        print(f"{((x + 1) / map.width() * 100):.2f} %")

    print(f"min path length: {min_length}")


if __name__ == "__main__":
    main()
