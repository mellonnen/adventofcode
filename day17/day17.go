package main

import (
	"bufio"
	"fmt"
	"os"
)

// IO
var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)

// POS structs
type pos3D struct {
	x int
	y int
	z int
}

type pos4D struct {
	x int
	y int
	z int
	w int
}

// sets of active cubes in bothe 3D and 4D
var activeCubes3D = make(map[pos3D]bool) // part 1
var activeCubes4D = make(map[pos4D]bool) // part 2

func simulate3D() {
	fmt.Println(len(activeCubes3D))
	simRes := make(map[pos3D]bool)
	// 6 cycles will activate cubes in a 20 x 20 x 13 rectangle
	// propagated from the original slice
	for x := -6; x < 14; x++ {
		for y := -6; y < 14; y++ {
			for z := -6; z < 7; z++ {
				activeNeighbors := 0
				// check the Moore neighborhood
				for _, dx := range []int{-1, 0, 1} {
					for _, dy := range []int{-1, 0, 1} {
						for _, dz := range []int{-1, 0, 1} {
							// ignore the cube at (x,y,z)
							if dx != 0 || dy != 0 || dz != 0 {

								if activeCubes3D[pos3D{x + dx, y + dy, z + dz}] {
									activeNeighbors++
								}
							}
						}
					}
				}
				pos := pos3D{x, y, z}
				// Game of life rules
				if !activeCubes3D[pos] && activeNeighbors == 3 {
					simRes[pos] = true
				}
				if activeCubes3D[pos] && (activeNeighbors == 2 || activeNeighbors == 3) {
					simRes[pos] = true
				}
			}
		}
	}
	activeCubes3D = simRes // update state of the set
}

// run the 3D simulation for 6 cycles, returns answer
func p1() int {
	for i := 0; i < 6; i++ {
		simulate3D()
	}
	return len(activeCubes3D)
}
func simulate4D() {
	fmt.Println(len(activeCubes4D))
	simRes := make(map[pos4D]bool)
	// 6 cycles will now instead activate cubes in a 20 x 20 x 7 x 7 hyperrectangle
	// propgated from the intial slice
	for x := -6; x < 14; x++ {
		for y := -6; y < 14; y++ {
			for z := -6; z < 7; z++ {
				for w := -6; w < 7; w++ {
					activeNeighbors := 0

					// Moore neighborhood
					for _, dx := range []int{-1, 0, 1} {
						for _, dy := range []int{-1, 0, 1} {
							for _, dz := range []int{-1, 0, 1} {
								for _, dw := range []int{-1, 0, 1} {
									if dx != 0 || dy != 0 || dz != 0 || dw != 0 {

										if activeCubes4D[pos4D{x + dx, y + dy, z + dz, w + dw}] {
											activeNeighbors++
										}
									}
								}
							}
						}
					}
					pos := pos4D{x, y, z, w}

					if !activeCubes4D[pos] && activeNeighbors == 3 {
						simRes[pos] = true
					}
					if activeCubes4D[pos] && (activeNeighbors == 2 || activeNeighbors == 3) {
						simRes[pos] = true
					}
				}
			}
		}
	}
	activeCubes4D = simRes
}

// runs 4D simulation for 6 cycles , and returns result
func p2() int {
	for i := 0; i < 6; i++ {
		simulate4D()
	}

	return len(activeCubes4D)
}
func main() {
	// Proces input
	y := 0
	for io.Scan() {
		row := []rune(io.Text())
		for x, state := range row {
			if state == '#' {
				activeCubes3D[pos3D{x, y, 0}] = true
				activeCubes4D[pos4D{x, y, 0, 0}] = true
			}
		}
		y++
	}

	fmt.Println("<==Part 1==>")
	fmt.Println(p1())
	fmt.Println("<==Part 2==>")
	fmt.Println(p2())
}
