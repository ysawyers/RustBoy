package interpreter

import "errors"

type stack[T any] struct {
	buffer []T
	length int
}

func (s *stack[T]) size() int {
	return s.length
}

func (s *stack[T]) peek() (*T, error) {
	if s.length == 0 {
		return nil, errors.New("Cannot peek empty stack")
	}
	return &s.buffer[s.length-1], nil
}

func (s *stack[T]) push(v T) {
	s.buffer = append(s.buffer, v)
	s.length++
}

func (s *stack[T]) pop() (*T, error) {
	if s.length == 0 {
		return nil, errors.New("Cannot pop from empty stack")
	}

	top, _ := s.peek()
	s.buffer = s.buffer[:s.length-1]
	s.length--
	return top, nil
}
