package jsonparser

import (
	"haversine/jsonparser/jsonparse"
	"strings"
)

// Hello returns a greeting for the named person.
func Parse(json string) jsonparse.JDocument {
	json = strings.ReplaceAll(json, " ", "")
	json = strings.ReplaceAll(json, "\r\n", "")

	return jsonparse.Parse(json)
}
