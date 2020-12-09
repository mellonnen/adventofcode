package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// IO
var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)

// NODE Struct

type Node struct {
	opcode   string
	modifier int
}

// GRAPH STRUCT
type Graph struct {
	nodes map[int]*Node
	edges map[int]int // maps line number -> line number
  reverseEdges map[int][]int
}


// GRAPH FUNCTIONS

func (g *Graph) addNode(line int, opcode string, modifier int) {

  var node Node	
	node.modifier = modifier
	node.opcode = opcode

	if g.nodes == nil {
		g.nodes = make(map[int]*Node)
	}
	g.nodes[line] = &node
}

func (g *Graph) addEdge(line1, line2 int) {
	if g.edges == nil {
		g.edges = make(map[int]int)
	}
  if g.reverseEdges == nil {
		g.reverseEdges = make(map[int][]int)
	}
  // we can have more than one nodes that connnect out node upstream 
  // if we move "backwards"

  // o o
  //  \|
  //   o <- line2

	g.edges[line1] = line2
	g.reverseEdges[line2] = append(g.reverseEdges[line2], line1) 
}

func (g *Graph) accumulate() int {
	executed := make(map[int]bool)
	accumulator := 0

	i := 0

	for !executed[i] {
    executed[i] = true
		node := g.nodes[i]
		if node.opcode == "acc" {
			accumulator += node.modifier
		}
		next, ok := g.edges[i]
		if !ok {
			break
		}
		i = next
	}
	return accumulator
}

// fixes the program by searching end/ star component of graph
func (g *Graph) fixProgram() {
  
  // BFS to map out the end component
  //(the graph component that contains the last instruction) 
  endComponent := make(map[int]bool)
  q := list.New()
  q.PushBack(len(g.nodes) - 1)

  for q.Len() != 0 {

    queElem := q.Front()
		q.Remove(queElem)

		// have to get value and than do type assertion
		node := queElem.Value.(int)

    endComponent[node] = true
    neighbors := g.reverseEdges[node]
    for _, neighbor := range neighbors {
				if !endComponent[neighbor] {
					q.PushBack(neighbor)
				}
			}       
  }
    
  // traverse the start component
  j := 0
  for {
      if g.nodes[j].opcode != "acc"{
        var target int // the target (next node in graph) if we switch
        switch g.nodes[j].opcode {
        case "nop":
          target = j + g.nodes[j].modifier
          if target > len(g.nodes) || target < 0 {
            continue
          }
        case "jmp":
          target = j + 1
        } 

        // checks if target is in endComponent -> switch
        if endComponent[target] {
          g.edges[j] = target
          break 
        }
      }
      j = g.edges[j]
    }
}

var g Graph

func main() {

  // Populate graph
	line := 0
	for  io.Scan() {
		instruction := io.Text()
		split := strings.Split(instruction, " ")
		opcode := split[0]
		modifier, _ := strconv.Atoi(split[1])
		g.addNode(line, opcode, modifier)

		if opcode == "jmp" {
			g.addEdge(line, line + modifier)
		} else {
			g.addEdge(line, line+1)
		}
    line++
	}
  // remove edges to non existing nodes
  delete(g.edges,line - 1)
  delete(g.reverseEdges,line)

  // Part 1
  fmt.Println("Part 1: accumulated = " + strconv.Itoa(g.accumulate()))

  // Part 2
  g.fixProgram()
  fmt.Println("Part 2: accumulated = " + strconv.Itoa(g.accumulate()))
}
