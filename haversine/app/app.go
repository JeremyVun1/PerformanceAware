package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"haversine/app/generator"
	"haversine/app/util"
	"haversine/jsonparser"
	"log"

	"net/http"
	_ "net/http/pprof"
	"time"
)

func main() {
	go func() {
		log.Println(http.ListenAndServe("localhost:8080", nil))
	}()

	generateFlag := flag.Bool("generate", false, "Generate haversine pairs")
	filenameFlag := flag.String("f", "test_data.json", "Filename")
	size := flag.Int("size", 100, "generator size")
	seed := flag.Int("seed", 1, "generator seed")

	parseFlag := flag.Bool("parse", true, "Parse the haversine pairs json")
	processFlag := flag.Bool("process", false, "Parse and process the haversine pairs")
	outputFlag := flag.Bool("output", false, "Write the parsed json to file")
	golibFlag := flag.Bool("golib", false, "Deserialize using go json lib")

	flag.Parse()
	filename := "data/" + *filenameFlag

	if *generateFlag {
		start := time.Now()
		generator.GenerateData(filename, *size, *seed)
		fmt.Printf("finished generating in %v ms\n", time.Since(start).Milliseconds())
		return
	}

	fileChannel := make(chan []byte, 512)
	if *golibFlag {
		start := time.Now()
		var jsonObj map[string]interface{}
		json.Unmarshal(util.ReadFile(filename), &jsonObj)
		fmt.Printf("finished Parsing in %v ms\n", time.Since(start).Milliseconds())
	} else if *parseFlag {
		start := time.Now()
		go util.StreamFile(filename, 4096, &fileChannel)
		obj := jsonparser.Deserialize(&fileChannel)
		if *outputFlag {
			var buffer bytes.Buffer
			obj.ToJson(&buffer)
			util.WriteFile("data/"+*filenameFlag+"result.json", buffer.Bytes())
		}
		fmt.Printf("finished Parsing in %v ms\n", time.Since(start).Milliseconds())
		// input := bufio.NewScanner(os.Stdin)
		// input.Scan()
	} else if *processFlag {
		start := time.Now()
		go util.StreamFile(filename, 4096, &fileChannel)
		//Process(&channel)
		fmt.Printf("finished Parsing in %v ms\n", time.Since(start).Milliseconds())
	}
}

func ParseUsinGoLibJson(inChannel *chan []byte) {

}

////
// BENCHMARKS (10,000,000 pairs)
// writing output to file
// original, 		25s, 16.5gb
// improved, 		16s, 6.5gb

// without writing output to file
// .net json 		7.8s 6.5gb
// newtonsoft json	55s  19.5gb
// golang json		9.5s 4gb
// improved 		9.7s 4gb
