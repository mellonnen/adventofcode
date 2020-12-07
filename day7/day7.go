package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// IO
var file, _ = os.Open("./input")
var io = bufio.NewScanner(file)

// DIRCTED GRAPH STRUCT

type Graph struct {
	nodes map[string]bool     //set of nodes
	edges map[string][]string // map node -> node
}

// GRAPH FUNCTIONS

func (g *Graph) addNode(n string) {
	if g.nodes == nil {
		g.nodes = make(map[string]bool)
	}
	g.nodes[n] = true
}

func (g *Graph) addEdge(n1, n2 string) {
	if g.edges == nil {
		g.edges = make(map[string][]string)
	}
	g.edges[n1] = append(g.edges[n1], n2)
}

// TUPLE STRUCT

type TreeTuple struct {
	bagCount int
	treeNode *Tree
}

// TREE STRUCT

type Tree struct {
	key      string
	children []TreeTuple
}

// TREE FUNCTIONS

func (t *Tree) addChild(bagCount int, child *Tree) {
	var tuple TreeTuple
	tuple.bagCount = bagCount
	tuple.treeNode = child
	t.children = append(t.children, tuple)
}

// used to solve part 2
func (node *Tree) inOrderTreeTraversal() int {
	bags := 1
	for _, tuple := range node.children {
		bagCount := tuple.bagCount
		childBagcount := tuple.treeNode.inOrderTreeTraversal()
		bags += bagCount * childBagcount
	}
	return bags
}

var treeNodes = make(map[string]*Tree) // tree nodes
var g Graph                            // directed graph

var re = regexp.MustCompile("[1-9]")

func main() {

	// populate graph and tree
	for io.Scan() {
		line := io.Text()

		// clean up the line, removing stuff
		line = strings.ReplaceAll(line, "bags", "")
		line = strings.ReplaceAll(line, "bag", "")
		line = strings.ReplaceAll(line, ".", "")

		// split on contain since input is: {parent bag} "contain" {child bag} {child bag} ...
		split := strings.Split(line, "contain")

		parentString := strings.TrimSpace(split[0])

		// create parent tree node if it does not exist
		if _, ok := treeNodes[parentString]; !ok {
			var parent Tree
			parent.key = parentString
			treeNodes[parentString] = &parent
		}

		// get parent tree node
		parent := treeNodes[parentString]

		// add parent graph node
		g.addNode(parentString)

		// split on "," since that seperates the child nodes

		children := strings.Split(split[1], ",")

		// proccess all child nodes
		for _, childString := range children {

			if strings.Contains(childString, "no other") {
				break
			}

			// find bagcount in input string,convert to int 
      // the regex works since there can only be single digit bags of one color whithin a parent bag
			countString := re.FindString(childString) 
			bagCount, _ := strconv.Atoi(countString)

			// remove number bag count string
			childString = re.ReplaceAllString(childString, "")
			childString = strings.TrimSpace(childString)

			// add node/edges to graph
			g.addNode(childString)
			// we want to map current bag -> bags that current bag
			//                               can be directly cotainted within

			g.addEdge(childString, parentString)

			// creates child tree node if it does not exist

			if _, ok := treeNodes[childString]; !ok {
				var child Tree
				child.key = childString
				treeNodes[childString] = &child
			}

			// adds child node to tree parent node 
			parent.addChild(bagCount, treeNodes[childString])
		}
	}

	// find the size of the component that contains "shiny gold"
	// by using BFS

	// -1 since we dont want to count the "shiny gold node"

	componentSize := -1
	queue := list.New()
	visited := make(map[string]bool)

	// append start node, "shiny gold"
	queue.PushBack("shiny gold")

	for queue.Len() != 0 {
		// pop of queue
		queElem := queue.Front()
		queue.Remove(queElem)

		// have to get value and than do type assertion
		node := queElem.Value.(string)

		if !visited[node] {
			// mark as visited
			visited[node] = true

			// get neighbours and add new nodes to queue
			neighbors := g.edges[node]
			for _, neighbor := range neighbors {
				if !visited[neighbor] {
					queue.PushBack(neighbor)
				}
			}
			// increment the size of the component
			componentSize++
		}
	}
	// part 1 soulution
	fmt.Println(componentSize)

	// part 2 soulution (-1 i since we do not want to count the "shiny gold" bag)
	fmt.Println(treeNodes["shiny gold"].inOrderTreeTraversal() - 1)
}
