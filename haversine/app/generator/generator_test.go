package generator

import (
	"testing"
)

func TestWrapNegativeOverflow(t *testing.T) {
	var point float32 = -150
	var translation float32 = -150
	overflow := (-150 - 150) + 180
	expected := 180 + overflow
	actual := wrap(point, translation, 180)

	if int(actual) != expected {
		t.Errorf("overflow %d expected %d", int(overflow), expected)
	}
}

func TestWrapPositiveveOverflow(t *testing.T) {
	var point float32 = 150
	var translation float32 = 150
	overflow := (150 + 150) - 180
	expected := -180 + overflow
	actual := wrap(point, translation, 180)

	if int(actual) != expected {
		t.Errorf("overflow %d expected %d", int(overflow), expected)
	}
}

func TestWrapZero(t *testing.T) {
	var point float32 = -125.68
	var translation float32 = 0
	var expected float32 = -125.68
	actual := wrap(point, translation, 180)

	if actual != expected {
		t.Errorf("actual %f expected %f", actual, expected)
	}
}
