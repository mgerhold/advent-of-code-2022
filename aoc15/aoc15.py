import re
from collections import defaultdict
from typing import NamedTuple


class Point(NamedTuple):
    x: int
    y: int

    def __str__(self) -> str:
        return f"(x={self.x},y={self.y})"


class Area(NamedTuple):
    top_left: Point
    bottom_right: Point

    def __str__(self) -> str:
        return f"[{self.top_left} -> {self.bottom_right}]"

    def dimensions(self) -> Point:
        return Point(self.bottom_right.x - self.top_left.x + 1, self.bottom_right.y - self.top_left.y + 1)


def manhattan_distance(a: Point, b: Point) -> int:
    result: int = abs(b.x - a.x) + abs(b.y - a.y)
    return result


class Sensor():
    def __init__(self, position: Point, beacon_position: Point) -> None:
        self.__position = position
        self.__beacon_position = beacon_position
        self.__manhattan_distance = manhattan_distance(
            self.__position, self.__beacon_position)

    def manhattan_distance(self) -> int:
        return self.__manhattan_distance

    # returns the range [start; end] that is covered by this sensor in the given row
    def range_in_row(self, row: int) -> tuple[int, int] | None:
        height_offset: int = abs(row - self.__position.y)
        reach = self.__manhattan_distance - height_offset
        if reach <= 0:
            # this sensor doesn't cover any area in the given row
            return None
        return (self.__position.x - reach, self.__position.x + reach)

    def position(self) -> Point:
        return self.__position

    def beacon_position(self) -> Point:
        return self.__beacon_position


def num_covered_in_row(sensors: list[Sensor], y: int) -> int:
    sensors_in_row: set[int] = set()
    beacons_in_row: set[int] = set()

    start_x_values: defaultdict[int, int] = defaultdict(lambda: 0)
    end_x_values: defaultdict[int, int] = defaultdict(lambda: 0)

    # the following code is ugly, but it makes it possible to only
    # iterate the sensors once
    min_x: int | None = None
    max_x: int | None = None
    for sensor in sensors:
        range_ = sensor.range_in_row(y)
        if range_ is not None:
            start_x = range_[0]
            end_x = range_[1]
            if min_x is None or start_x < min_x:
                min_x = start_x
            if max_x is None or end_x > max_x:
                max_x = end_x
            start_x_values[start_x] += 1
            end_x_values[end_x] += 1
        sensor_position = sensor.position()
        if sensor_position.y == y:
            sensors_in_row.add(sensor_position.x)
        beacon_position = sensor.beacon_position()
        if beacon_position.y == y:
            beacons_in_row.add(beacon_position.x)

    assert min_x is not None
    assert max_x is not None

    active_ranges = 0
    num_covered = 0

    row = ""
    for x in range(min_x, max_x + 1):
        # add active ranges
        if x in start_x_values:
            active_ranges += start_x_values[x]

        # evaluate current x-position
        if x in beacons_in_row:
            row += "B"
            pass
        elif x in sensors_in_row:
            row += "S"
            pass
        elif active_ranges > 0:
            num_covered += 1
            row += "#"
        else:
            row += '.'

        # subtract active ranges
        if x in end_x_values:
            active_ranges -= end_x_values[x]

        assert active_ranges >= 0
    return num_covered


def covers_area(sensor: Sensor, area: Area) -> bool:
    min_x, max_x = area.top_left.x, area.bottom_right.x
    min_y, max_y = area.top_left.y, area.bottom_right.y
    for y in range(min_y, max_y + 1):
        range_ = sensor.range_in_row(y)
        if range_ is None or range_[0] > min_x or range_[1] < max_x:
            return False
    return True


def find_free_position(
    sensors: list[Sensor],
    area: Area,
    show_output: bool,
    indentation: int = 0
) -> Point | None:
    dimensions = area.dimensions()
    if show_output:
        print(f"{' ' * indentation}looking at area {area}")
    if dimensions.x < 1 or dimensions.y < 1:
        return None
    for sensor in sensors:
        # check if a sensor totally covers the area
        if covers_area(sensor, area):
            # there is no free position inside this area
            return None

    if dimensions.x == 1 and dimensions.y == 1:
        return area.top_left

    # partition the area into four sub-areas and recursively check those
    middle = Point((area.top_left.x + area.bottom_right.x) // 2,
                   (area.top_left.y + area.bottom_right.y) // 2)
    sub_areas = (
        Area(top_left=area.top_left, bottom_right=middle),
        Area(top_left=Point(middle.x + 1, area.top_left.y),
             bottom_right=Point(area.bottom_right.x, middle.y)),
        Area(top_left=Point(area.top_left.x, middle.y + 1),
             bottom_right=Point(middle.x, area.bottom_right.y)),
        Area(top_left=Point(middle.x + 1, middle.y + 1),
             bottom_right=area.bottom_right),
    )
    for sub_area in sub_areas:
        result = find_free_position(
            sensors, sub_area, show_output, indentation + 2)
        if result is not None:
            return result
    return None


def main() -> None:
    PART_1_ROW = 2000000
    MAX_X = 4000000
    MAX_Y = MAX_X
    FILENAME = "real_input.txt"

    # regex generated by ChatGPT LUL
    regex = r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"

    sensors: list[Sensor] = list()

    with open(FILENAME) as file:
        for line in file:
            match_ = re.match(regex, line)
            assert match_ and len(match_.groups()) == 4
            coords = [int(s) for s in match_.groups()]
            sensor_position = Point(coords[0], coords[1])
            beacon_position = Point(coords[2], coords[3])
            sensors.append(Sensor(sensor_position, beacon_position))

    # part 1
    num_covered = num_covered_in_row(sensors, PART_1_ROW)
    print(f"there are {num_covered} spots covered in row {PART_1_ROW}")

    # part 2
    free_position = find_free_position(sensors, Area(top_left=Point(0, 0),
                                                     bottom_right=Point(MAX_X, MAX_Y)), show_output=False)
    assert isinstance(free_position, Point)
    print(f"found free position at {free_position}")
    result = free_position.x * 4000000 + free_position.y
    print(f"result is {result}")


if __name__ == "__main__":
    main()
