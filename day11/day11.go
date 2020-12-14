package main

import (
	"bufio"
	"fmt"
	"os"
)

// IO
var file,_ = os.Open("./input")
var io = bufio.NewScanner(file)


const rows = 98
const cols = 95
var grid = [rows][cols]rune{}

// FUNCTIONS
func deepcopy() [rows][cols]rune {
  var g = [rows][cols]rune{}
  for i := 0; i < rows; i++ {
   for j := 0; j < cols; j++ {
      g[i][j] = grid[i][j] 
   } 
  }
  return g
}

// Simulates for Part 1
func simulate1() {
  for {
    change := false
    gridCopy := deepcopy() // copies grid since all the seats are simultaneously updated
    for y := 0; y < rows; y++ {
      for x := 0; x < cols; x++ {
        occupied := 0
        // check the eight adjecent positions
        for _, i := range []int{-1, 0, 1} {
          for _, j := range []int{-1, 0, 1} {
            if !(i == 0 && j == 0) {
              dx := x + j
              dy := y + i
              if 0 <= dx && dx < cols && 0 <= dy && dy < rows && grid[dy][dx] == '#' {
                occupied++
              }
            }  
          }
        }
        if grid[y][x] == 'L' {
          if occupied == 0 {
            gridCopy[y][x] = '#'
            change = true // we updated the state of that seat 
          }
        } else if grid[y][x] == '#' {
          if occupied >= 4 {
            gridCopy[y][x] = 'L'
            change = true
          }
        }
      } 
    }
    if !change { // if the state of all seats are unchanged we are done simulating
      break
    }
    grid = gridCopy // update the state of the grid
  }

}

func simulate2() {
  for {
    change := false
    gridCopy := deepcopy()
    for y := 0; y < rows; y++ {
      for x := 0; x < cols; x++ {
        occupied := 0
        // check all line of sights from the seat
        for _, i := range []int{-1, 0, 1} {
          for _, j := range []int{-1, 0, 1} {
            if !(i == 0 && j == 0) {
              dx := x + j
              dy := y + i
              for 0 <= dx && dx < cols && 0 <= dy && dy < rows && grid[dy][dx] == '.' { // Iterate until we find a seat
                dx += j
                dy += i
              } 
              if 0 <= dx && dx < cols && 0 <= dy && dy < rows && grid[dy][dx] == '#' { // check if seat is occupied
                  occupied++
              }
            }  
          }
        }
        if grid[y][x] == 'L' {
          if occupied == 0 {
            gridCopy[y][x] = '#'
            change = true
          }
        } else if grid[y][x] == '#' {
          if occupied >= 5 {
            gridCopy[y][x] = 'L'
            change = true
          }
        }
      } 
    }
    if !change {
      break
    }
    grid = gridCopy 
  }

}

func main() {
  
  y := 0
  for io.Scan() {
    runes := []rune(io.Text()) 
    for x := 0; x < len(runes); x++{
      grid[y][x] = runes[x]
    }
    y++
  }

  temp := deepcopy() // make copy to use for Part 2

  // Part 1

  simulate1()
  ans1 := 0
  for y := 0; y < rows; y++ {
    for x := 0; x < cols; x++ {
      if grid[y][x] == '#' {
        ans1++
      }
    }
  }
  fmt.Println(ans1)
  
  grid = temp // reset grid

  // Part 2

  simulate2()
  ans2 := 0
  for y := 0; y < rows; y++ {
    for x := 0; x < cols; x++ {
      if grid[y][x] == '#' {
        ans2++
      }
    }
  }
  fmt.Println(ans2)
}
 
