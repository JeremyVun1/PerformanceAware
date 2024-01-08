package stack

type Stack struct {
	items []rune
}

func (s *Stack) Push(item rune) {
	s.items = append(s.items, item)
}

func (s *Stack) Pop() rune {
	if len(s.items) == 0 {
		return rune(0)
	}

	item := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]

	return rune(item)
}
