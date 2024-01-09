package generator

import (
	"bufio"
	"bytes"
	"fmt"
	"haversine/app/util"
	"math"
	"math/rand"
	"os"
	"strconv"
)

func GenerateData(filename string, size int, seed int) {
	fmt.Printf("Generating %d pairs\n", size)
	r := rand.New(rand.NewSource(int64(seed)))

	f, err := os.Create(filename)
	util.Check(err)
	writer := bufio.NewWriter(f)
	writer.WriteString("{\"pairs\":[")
	writer.Flush()

	var buffer bytes.Buffer
	sum := 0
	clusterPoint := getRandomHaversineLine(r)
	//clusterPoint := HaversineLine{x0: 0.0, x1: 0.0, y0: 0.0, y1: 0.0}
	for i := 0; i < size; i++ {
		if i%1000 == 0 {
			clusterPoint = getRandomHaversineLine(r)
		}

		line := getRandomClusteredHaversineLine(r, &clusterPoint)

		buffer.WriteString("{\"x0\":")
		buffer.WriteString(strconv.FormatFloat(float64(line.x0), 'f', 8, 32))
		buffer.WriteString(",\"x1\":")
		buffer.WriteString(strconv.FormatFloat(float64(line.x1), 'f', 8, 32))
		buffer.WriteString(",\"y0\":")
		buffer.WriteString(strconv.FormatFloat(float64(line.y0), 'f', 8, 32))
		buffer.WriteString(",\"y1\":")
		buffer.WriteString(strconv.FormatFloat(float64(line.y1), 'f', 8, 32))
		buffer.WriteRune('}')
		if i < size-1 {
			buffer.WriteRune(',')
		}

		writer.Write(buffer.Bytes())
		sum += 1

		buffer.Reset()
	}

	writer.WriteString("]}")
	writer.Flush()

	fmt.Printf("Generated %d random haversine pairs with seed %d to %s\n", sum, seed, filename)
}

func getRandomClusteredHaversineLine(r *rand.Rand, cluster *HaversineLine) HaversineLine {
	line := getRandomHaversineLine(r)
	line.x0 = wrap(line.x0/2, cluster.x0, 180)
	line.x1 = wrap(line.x1/2, cluster.x0, 180)
	line.y0 = wrap(line.y0/2, cluster.y0, 90)
	line.y1 = wrap(line.y1/2, cluster.y0, 90)

	return line
}

func getRandomHaversineLine(r *rand.Rand) HaversineLine {
	return HaversineLine{
		x0: r.Float32() + float32(179-r.Int31n(358)),
		x1: r.Float32() + float32(179-r.Int31n(358)),
		y0: r.Float32() + float32(89-r.Int31n(178)),
		y1: r.Float32() + float32(89-r.Int31n(178)),
	}
}

func wrap(point float32, translation float32, clamp float32) float32 {
	// 240 / 180 -> 180 - 60 = 120 * -1
	// -240 / -180 -> 180 - 60 = 120
	point += translation
	isPos := point > 0
	point = float32(math.Abs(float64(point)))

	if point > clamp {
		overflow := point - clamp
		point = clamp - overflow
		if isPos {
			point *= -1
		}
		return point
	} else if isPos {
		return point
	} else {
		return -point
	}
}

type HaversineLine struct {
	x0 float32
	x1 float32
	y0 float32
	y1 float32
}
