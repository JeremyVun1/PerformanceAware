package generator

import (
	"bufio"
	"bytes"
	"fmt"
	"haversine/app/util"
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
	for i := 0; i < size; i++ {
		x0 := r.Float32() + float32(179-r.Int31n(358))
		x1 := r.Float32() + float32(179-r.Int31n(358))
		y0 := r.Float32() + float32(89-r.Int31n(178))
		y1 := r.Float32() + float32(89-r.Int31n(178))
		buffer.WriteString("{\"x0\":")
		buffer.WriteString(strconv.FormatFloat(float64(x0), 'f', 8, 32))
		buffer.WriteString(",\"x1\":")
		buffer.WriteString(strconv.FormatFloat(float64(x1), 'f', 8, 32))
		buffer.WriteString(",\"y0\":")
		buffer.WriteString(strconv.FormatFloat(float64(y0), 'f', 8, 32))
		buffer.WriteString(",\"y1\":")
		buffer.WriteString(strconv.FormatFloat(float64(y1), 'f', 8, 32))
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
