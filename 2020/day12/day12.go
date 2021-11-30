package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

// IO
var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)

// boat data (Part 1)
var x = 0
var y = 0
var dir = 'E'
var turnLeft = map[rune]rune{'E': 'N', 'S': 'E', 'W': 'S', 'N': 'W'}
var turnRight = map[rune]rune{'E': 'S', 'S': 'W', 'W': 'N', 'N': 'E'}

// boat data (part 2)
var waypointX = 10
var waypointY = 1
var boatX = 0
var boatY = 0

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func navigate(command string) {
	action := command[0]
	input, _ := strconv.Atoi(command[1:])

	switch action {
	case 'E':
		x += input
	case 'S':
		y -= input
	case 'W':
		x -= input
	case 'N':
		y += input
	case 'L':
		times := input / 90
		for i := 0; i < times; i++ {
			dir = turnLeft[dir]
		}
	case 'R':
		times := input / 90
		for i := 0; i < times; i++ {
			dir = turnRight[dir]
		}
	case 'F':
		switch dir {
		case 'E':
			x += input
		case 'S':
			y -= input
		case 'W':
			x -= input
		case 'N':
			y += input
		}
	}
}

func waypointNavigate(command string) {
	action := command[0]
	input, _ := strconv.Atoi(command[1:])

	switch action {
	case 'E':
		waypointX += input
	case 'S':
		waypointY -= input
	case 'W':
		waypointX -= input
	case 'N':
		waypointY += input
	case 'L':
		times := input / 90
		for i := 0; i < times; i++ {
			temp := waypointX
			waypointX = -waypointY
			waypointY = temp
		}
	case 'R':
		times := input / 90
		for i := 0; i < times; i++ {
			temp := waypointX
			waypointX = waypointY
			waypointY = -temp
		}
	case 'F':
		boatX += input * waypointX
		boatY += input * waypointY

	}
}

func main() {

	// Part 1
	for io.Scan() {
		command := io.Text()
		navigate(command)
		waypointNavigate(command)
	}

	fmt.Println(abs(x) + abs(y))
	fmt.Println(abs(boatX) + abs(boatY))

}
