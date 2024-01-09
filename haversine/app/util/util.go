package util

import (
	"bufio"
	"fmt"
	"io"
	"os"
)

func ReadFile(filename string) []byte {
	data, err := os.ReadFile(filename)
	Check(err)
	return data
}

func StreamFile(filename string, size int, channel *chan []byte) {
	file, err := os.Open(filename)
	Check(err)

	reader := bufio.NewReader(file)
	i := 0
	chunks := 0
	chunk_size := 65536
	for {
		buffer := make([]byte, chunk_size)
		_, err := reader.Read(buffer)
		if err == io.EOF {
			fmt.Println("Finished reading file")
			close(*channel)
			file.Close()
			break
		} else if err != nil {
			close(*channel)
			panic(err)
		}

		n := find_last_rune(buffer, '}') + 1
		i += n

		*channel <- buffer[:n]
		file.Seek(int64(i), 0)

		chunks += 1
	}
	fmt.Printf("streamed chunks %d from file\n", chunks)
}

func find_last_rune(buffer []byte, c rune) int {
	i := len(buffer) - 1
	for i >= 0 {
		if rune(buffer[i]) == c {
			return i
		}
		i -= 1
	}

	println(string(buffer))
	panic("could not trim buffer")
}

func WriteFile(filename string, buffer []byte) {
	fmt.Printf("Writing json to file %s\n", filename)
	os.WriteFile(filename, buffer, 0644)
}

func Check(err error) {
	if err != nil {
		panic(err)
	}
}
