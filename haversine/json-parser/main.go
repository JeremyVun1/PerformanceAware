package main

import (
	"fmt"
	"haversine/json-parser/jsonparse"
	"haversine/json-parser/util"
)

func main() {
	data := util.ReadFile("test.json")
	content := string(data)

	obj := jsonparse.Deserialize(content)
	fmt.Println(obj.ToJson())
}
