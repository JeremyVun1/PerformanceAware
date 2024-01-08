package jsonparse

import (
	"fmt"
	"strings"
)

/*
Parser models
*/
type JToken interface {
	ToJson() string
	getSize() int
}

// Value
type JString struct {
	Value string
}

func (jString JString) ToJson() string {
	return fmt.Sprintf("\\\"%s\\\"", jString.Value)
}
func (jString JString) getSize() int {
	return 1
}

type JFloat struct {
	Value float32
}

func (jFloat JFloat) ToJson() string {
	return fmt.Sprintf("%g", jFloat.Value)
}
func (jFloat JFloat) getSize() int {
	return 1
}

type JInt struct {
	Value int
}

func (jInt JInt) ToJson() string {
	return fmt.Sprintf("%d", jInt.Value)
}
func (jInt JInt) getSize() int {
	return 1
}

// Array
type JArray struct {
	Children []JToken
	size     int
}

func (jArray JArray) ToJson() string {
	var tokens []string
	for _, child := range jArray.Children {
		if child == nil {
			tokens = append(tokens, "null")
		} else {
			tokens = append(tokens, child.ToJson())
		}
	}
	return fmt.Sprintf("[%s]", strings.Join(tokens, ","))
}
func (jArray JArray) getSize() int {
	return jArray.size
}

// Object
type JObject struct {
	Keys   []string
	Values []JToken
	//Children map[string]JToken
	size int
}

func (jObject JObject) ToJson() string {
	var tokens []string
	for i, key := range jObject.Keys {
		value := jObject.Values[i]
		if value != nil {
			tokens = append(tokens, fmt.Sprintf("\\\"%s\\\":%s", key, value.ToJson()))
		} else {
			tokens = append(tokens, fmt.Sprintf("\\\"%s\\\":%s", key, "null"))
		}

	}
	return fmt.Sprintf("{%s}", strings.Join(tokens, ","))
}

func (jObject JObject) getSize() int {
	return jObject.size
}

type JDocument struct {
	Value JToken
}

func (jDocument JDocument) ToJson() string {
	return jDocument.Value.ToJson()
}
func (jDocument JDocument) getSize() int {
	return jDocument.Value.getSize()
}

/*
Lexer Tokens
*/
type jToken struct {
	Type  JTokenType
	Value []rune
}

type JTokenType byte

const (
	JStartArray JTokenType = 0
	JEndArray   JTokenType = 1
	JArrayToken JTokenType = 2

	JStartObject JTokenType = 3
	JEndObject   JTokenType = 4
	JObjectToken JTokenType = 5

	JStringToken JTokenType = 6
	JFloatToken  JTokenType = 7
	JIntToken    JTokenType = 8
	JColonToken  JTokenType = 9
	JCommaToken  JTokenType = 10
	JNullToken   JTokenType = 11
)
