package main

import (
	"fmt"
	"haversine/app/util"
	"haversine/jsonparser"
	. "haversine/jsonparser/jsonparse"
)

func main() {
	json := string(util.ReadFile("test.json"))
	fmt.Printf("Deserializing: %s\n", json)
	jDoc := jsonparser.Parse(json)
	fmt.Println(jDoc.ToJson())

	jObject := jDoc.Value.(JObject)
	x := GetItem[JString](jObject, "string")
	fmt.Println(x.Value)

	strArr := GetItem[JArray](jObject, "strArray")
	strVal := GetItemAt[JString](strArr, 0)
	fmt.Println(strVal.Value)

	objArr := GetItem[JArray](jObject, "objArray")
	objVal := GetItemAt[JObject](objArr, 0)
	y := GetItem[JString](objVal, "name").Value
	fmt.Println(y)
}
