import time
import pygame


class Map():
    def __init__(self, filename: str) -> None:
        with open(filename) as file:
            self._rocks: set[tuple[int, int]] = set()
            self._sand: set[tuple[int, int]] = set()
            for line in file:
                points = line.split(" -> ")
                assert len(points) > 1, "expected at least two points per path"
                for i in range(0, len(points) - 1):
                    x1, y1 = [int(s) for s in points[i].split(",")]
                    x2, y2 = [int(s) for s in points[i + 1].split(",")]
                    if x1 == x2:
                        for y in range(min(y1, y2), max(y1, y2) + 1):
                            self._rocks.add((x1, y))
                    elif y1 == y2:
                        for x in range(min(x1, x2), max(x1, x2) + 1):
                            self._rocks.add((x, y1))
                    else:
                        assert False, "invalid path"
            self._min_x = min(map(lambda pos: pos[0], self._rocks))
            self._max_x = max(map(lambda pos: pos[0], self._rocks))
            self._min_y = min(map(lambda pos: pos[1], self._rocks))
            self._max_y = max(map(lambda pos: pos[1], self._rocks))
            self._active_sand: None | tuple[int, int] = None

    def has_active_sand(self) -> bool:
        return self._active_sand is not None

    # returns True if the spawn location is already blocked
    def spawn_sand(self, part2: bool) -> bool:
        SPAWN_LOCATION = (500, 0)
        if not self._is_free(SPAWN_LOCATION, part2):
            return True
        assert not self.has_active_sand(), "cannot spawn more than one sand at a time"
        self._active_sand = SPAWN_LOCATION
        return False

    # returns True if the sand left the map
    def update(self, quick_mode: bool = False, part2: bool = False) -> bool:
        if not self.has_active_sand():
            return False
        assert isinstance(self._active_sand, tuple) and list(
            map(type, self._active_sand)) == [int, int]
        while True:
            possible_positions = [
                (self._active_sand[0], self._active_sand[1] + 1),  # down
                (self._active_sand[0] - 1,
                 self._active_sand[1] + 1),  # down left
                (self._active_sand[0] + 1,
                 self._active_sand[1] + 1),  # down right
            ]
            moved = False
            for position in possible_positions:
                if self._is_free(position, part2):
                    self._active_sand = position

                    # for part 1, check if the sand left the map
                    if not part2 and self._active_sand[1] >= self._max_y:
                        # the sand left the map
                        self._active_sand = None
                        return True

                    if not quick_mode:
                        return False
                    else:
                        moved = True
                        break

            if not moved:
                # we could not move the sand - it comes to rest
                self._sand.add(self._active_sand)
                self._active_sand = None
                return False

    def _is_free(self, coords: tuple[int, int], part2: bool) -> bool:
        # floor
        if part2 and coords[1] == self._max_y + 2:
            return False

        return coords not in self._rocks and coords not in self._sand

    def _coords_to_rect(
        self,
        coords: tuple[int, int],
        render_size: tuple[int, int],
        part2: bool
    ) -> pygame.Rect:
        x, y = coords
        map_size = (self._max_x - self._min_x + 1, self._max_y +
                    3 if part2 else self._max_y + 1)
        rect_size = (render_size[0] / map_size[0],
                     render_size[1] / map_size[1])
        position = (rect_size[0] * (x - self._min_x), rect_size[1] * y)
        return pygame.Rect(position[0], position[1], rect_size[0], rect_size[1])

    def render(self, window: pygame.surface.Surface, render_size: tuple[int, int], part2: bool = False) -> None:
        SAND_COLOR = (240, 219, 125)
        ROCK_COLOR = (100, 100, 100)
        max_y = self._max_y + 2 if part2 else self._max_y
        for y in range(max_y + 1):
            for x in range(self._min_x, self._max_x + 1):
                point = (x, y)
                if point in self._rocks:
                    pygame.draw.rect(window, ROCK_COLOR,
                                     self._coords_to_rect(point, render_size, part2))
                elif point in self._sand:
                    pygame.draw.rect(window, SAND_COLOR,
                                     self._coords_to_rect(point, render_size, part2))
        if self.has_active_sand():
            assert isinstance(self._active_sand, tuple) and list(
                map(type, self._active_sand)) == [int, int]
            pygame.draw.rect(window, SAND_COLOR, self._coords_to_rect(
                self._active_sand, render_size, part2))

        if part2:
            for x in range(self._min_x, self._max_x + 1):
                point = (x, max_y)
                pygame.draw.rect(window, ROCK_COLOR,
                                 self._coords_to_rect(point, render_size, part2))


def main() -> None:
    PART2 = False
    QUICK_UPDATING = False
    FILENAME = "real_input.txt"
    WIDTH = 800
    HEIGHT = 600

    map = Map(FILENAME)
    pygame.init()
    window_size = (WIDTH, HEIGHT)
    window = pygame.display.set_mode(window_size)

    BLACK = (0, 0, 0)
    running = True
    num_spawned = 0
    result_found = False
    while running:
        # update
        if not result_found:
            if not map.has_active_sand():
                if map.spawn_sand(part2=PART2):
                    print("spawn location blocked!")
                    print(f"result: {num_spawned}")
                    result_found = True
                else:
                    num_spawned += 1
            if map.update(quick_mode=QUICK_UPDATING, part2=PART2):
                print(
                    f"we spawned {num_spawned} units of sand before sand left the map")
                print(f"result: {num_spawned - 1}")
                result_found = True
        # draw
        window.fill(BLACK)
        map.render(window, (WIDTH, HEIGHT), part2=PART2)
        pygame.display.update()
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN and event.key == pygame.K_ESCAPE:
                running = False

    pygame.quit()


if __name__ == "__main__":
    main()
