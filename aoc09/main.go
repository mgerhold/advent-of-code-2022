package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Command int

const (
	Up Command = iota
	Down
	Left
	Right
)

type Point struct {
	x int32
	y int32
}

func (p Point) String() string {
	return fmt.Sprintf("(%d, %d)", p.x, p.y)
}

func (p Point) ManhattanDistance(other Point) int32 {
	return Abs(other.x-p.x) + Abs(other.y-p.y)
}

func Abs(value int32) int32 {
	if value >= 0 {
		return value
	}
	return -value
}

func applyCommand(rope *Rope, command Command) {
	head := &rope[0]
	switch command {
	case Up:
		head.y++
	case Down:
		head.y--
	case Right:
		head.x++
	case Left:
		head.x--
	}
}

func dragTailAlong(rope *Rope) {
	for i := 1; i < len(rope); i++ {
		leadingKnot := rope[i-1]
		followingKnot := &rope[i]
		sameRow := leadingKnot.y == followingKnot.y
		if sameRow {
			if leadingKnot.x == followingKnot.x+2 {
				followingKnot.x++
			} else if leadingKnot.x == followingKnot.x-2 {
				followingKnot.x--
			}
			continue
		}

		sameColumn := leadingKnot.x == followingKnot.x
		if sameColumn {
			if leadingKnot.y == followingKnot.y+2 {
				followingKnot.y++
			} else if leadingKnot.y == followingKnot.y-2 {
				followingKnot.y--
			}
			continue
		}

		// the head either overlaps the tail or has moved diagonally
		manhattanDistance := followingKnot.ManhattanDistance(leadingKnot)
		if manhattanDistance != 2 && manhattanDistance != 3 && manhattanDistance != 4 {
			log.Fatal("unacceptable condition!!!!!!!")
		}
		if manhattanDistance >= 3 {
			if leadingKnot.x > followingKnot.x && leadingKnot.y > followingKnot.y {
				followingKnot.x++
				followingKnot.y++
			} else if leadingKnot.x > followingKnot.x && leadingKnot.y < followingKnot.y {
				followingKnot.x++
				followingKnot.y--
			} else if leadingKnot.x < followingKnot.x && leadingKnot.y > followingKnot.y {
				followingKnot.x--
				followingKnot.y++
			} else if leadingKnot.x < followingKnot.x && leadingKnot.y < followingKnot.y {
				followingKnot.x--
				followingKnot.y--
			} else {
				log.Fatal("UNACCEPTABLE!!!!")
			}
		}
	}
}

const numKnots = 10

type Rope [numKnots]Point

func printRope(rope *Rope) {
	for _, knot := range rope {
		fmt.Print(knot, " ")
	}
	fmt.Println()
}

func main() {
	file, err := os.Open("real_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var commands []Command
	for _, line := range lines {
		parts := strings.Split(line, " ")
		amount, err := strconv.ParseInt(parts[1], 10, 32)
		if err != nil {
			log.Fatal(err)
		}
		command := Up
		if parts[0] == "R" {
			command = Right
		} else if parts[0] == "L" {
			command = Left
		} else if parts[0] == "D" {
			command = Down
		} else if parts[0] != "U" {
			log.Fatal("invalid command")
		}
		for i := int64(0); i < amount; i++ {
			commands = append(commands, command)
		}
	}

	var rope Rope
	for i := range rope {
		rope[i] = Point{x: 0, y: 0}
	}
	printRope(&rope)

	visited := make(map[Point]struct{})
	visited[Point{x: 0, y: 0}] = struct{}{}

	for _, command := range commands {
		applyCommand(&rope, command)
		dragTailAlong(&rope)
		visited[rope[len(rope)-1]] = struct{}{}
		printRope(&rope)
	}

	fmt.Println(len(visited))
}
