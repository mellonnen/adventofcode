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

// STRUCTS
type tile struct {
  id int
  top string 
  left string
  bottom string
  right string
}

func (t *tile) rotate() {
  top := t.top
  left := t.left
  bottom := t.bottom
  right := t.right

  t.top = right
  t.left = top
  t.bottom = left
  t.right = bottom
}

func (t *tile) flip() {
  temp := t.left
  t.left = t.right
  t.right = temp
}

/* 
SYMETRIES OF TILE
+---+  +---+  +---o  o---+ 
|   |  |   |  |   |  |   |
o---+  +---o  +---+  +---+
 r0     r1     r2     r3
  
+-|-+  +---+
| | | -|---|-
+-|-+  +---+
 m1     m2  
     
+---+  +---+
| \ |  | / |
+---+  +---+
 d1     d2
*/


func r0(t tile)tile {
  return t
}

func r1(t tile)tile {
  tx := t
  tx.rotate()
  return tx
}

func r2(t tile)tile {
  tx := t
  tx.rotate()
  tx.rotate()
  return tx
}

func r3(t tile)tile {
  tx := t
  tx.rotate()
  tx.rotate()
  tx.rotate()
  return tx
}

func m1(t tile)tile {
  tx := t
  t.flip()
  return tx
}

func m2(t tile)tile {
 return r3(m1(t)) 
}

func d1(t tile) tile {
  return r2(m1(t)) 
}

func d2(t tile)tile {
  return r1(m2(t))
}

var symetries = []func(tile)tile{r0,r1,r2,r3,m1,m2,d1,d2}

func parseInput()[]tile {
  var tiles []tile
  i := 0
  var id int
  var top string
  var left string
  var bottom string
  var right string

  for io.Scan() {
    line := io.Text()
    if line == "" {
      tiles = append(tiles, tile{id,top,left,bottom,right}) 
      i = 0
      left, right = "", ""
    } else if i == 0 {
      id, _ = strconv.Atoi(line[5:len(line) - 2])
      i++
    } else if i == 1 {
      top = line
      left += string(line[0])                
      right += string(line[len(line) - 1])
      i++
    } else if i == 10 {
      bottom = line
      left += string(line[0])                
      right += string(line[len(line) - 1])
      i++
    } else {
      left += string(line[0])                
      right += string(line[len(line) - 1])
      i++
    }
  }
  tiles = append(tiles, tile{id,top,left,bottom,right}) 
  return tiles
}

func canPlaceTile (grid [][]tile, tile tile, x, y int)bool {
  var top,left bool

  if y == 0 {
    top = true
  } else {
    top = grid[y - 1][x].bottom == tile.top
  }
  if x == 0 {
    left = true
  } else {
    left = grid[y][x - 1].right == tile.left
  }

  return top && left
}

func arangeTiles (grid [][] tile, tiles []tile, used map[tile]bool)([][]tile,bool) {
  for y := range grid {
    for x := range grid {
      if (tile{}) == grid[y][x] {
        for _, masterTile := range tiles{
          if !used[masterTile] {
            t := masterTile
            // r0
            placed := canPlaceTile(grid,t,x,y)
            // r1 - r2
            if !placed {
              i := 0
              for i < 3 && !placed {
                t.rotate() 
                placed = canPlaceTile(grid,t,x,y) 
                i++;
              }
            } 


            // m1 = r0 m1 
            if !placed {
              t = masterTile // reset to r0
              t.flip()  
              placed = canPlaceTile(grid,t,x,y)
            } 

            // m2 = m1 r2
            if !placed {
              t.rotate()
              t.rotate()
              placed = canPlaceTile(grid,t,x,y)
            }
            

            // d1 = m2 r3
            if !placed {
              t.rotate()
              t.rotate()
              t.rotate()
              placed = canPlaceTile(grid,t,x,y)
            }

            // d2 = d1 r2
            if !placed {
              t.rotate()
              t.rotate()
              placed = canPlaceTile(grid,t,x,y)
            }
            if placed {
              grid[y][x] = t
              used[masterTile] = true
              
              grid,correct := arangeTiles(grid,tiles,used)
              if correct {
                return grid,correct
              } else {
                grid[y][x] = tile{}
                used[masterTile] = true
              }
            }
          } 
          return grid,false
        }
      }
    }
  }
  return grid,true 
}
func p1(tiles []tile)int {  
 grid := make([][]tile,12) 
 for i := range grid {
   grid[i] = make([]tile,12)
 }
 used := make(map[tile]bool)
 grid,_ = arangeTiles(grid,tiles,used)
 return grid[0][0].id * grid[0][11].id * grid[11][0].id * grid[11][11].id
}

func main() {
  tiles := parseInput()
  fmt.Println(p1(tiles))
}

