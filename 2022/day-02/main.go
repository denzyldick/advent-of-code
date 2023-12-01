package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, _ := os.ReadFile("./input")
	input := string(b)
	rounds := strings.Split(input, "\n")

	amount := 0
	all := []Game{
		{Chosen: Rock, Representation: []string{"A", "X"}},
		{Chosen: Paper, Representation: []string{"B", "Y"}},
		{Chosen: Siccor, Representation: []string{"C", "Z"}},
	}

	for _, v := range rounds {
		line := strings.Split(v, " ")
		if len(line) == 1 {
			continue
		}
		p2 := line[1]
		p1 := line[0]
		v2 := 0
		v1 := 0
		for _, o := range all {
			for _, c := range o.Representation {
				if c == p2 {
					v2 = o.Chosen
				}

				if c == p1 {
					v1 = o.Chosen
				}
			}
		}
		if v2 == 1 {
			// lose
			if v1 == 1 {
				amount = amount + 3
			}

			if v1 == 2 {
				amount = amount + 1
			}

			if v1 == 3 {
				amount = amount + 2
			}
		} else if v2 == 2 {
			// draw
			amount = amount + 3 + v1
		} else if v2 == 3 {
			// win
			if v1 == 1 {
				amount = amount + 2 + 6
			}
			if v1 == 2 {
				amount = amount + 3 + 6
			}
			if v1 == 3 {
				amount = amount + 1 + 6
			}
		}
	}
	fmt.Println(amount)
}

const (
	Rock   = 1
	Paper  = 2
	Siccor = 3
)

type Game struct {
	Chosen         int
	Representation []string
}
