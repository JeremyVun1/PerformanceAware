package jsonparse

import (
	"bytes"
	"strconv"
)

/*
Parser models
*/
type JElement interface {
	ToJson(buffer *bytes.Buffer)
}

// Value
type JString struct {
	Value string
}

func (jString JString) ToJson(buffer *bytes.Buffer) {
	buffer.WriteString("\"" + jString.Value + "\"")
}

type JFloat struct {
	Value float32
}

func (jFloat JFloat) ToJson(buffer *bytes.Buffer) {
	buffer.WriteString(strconv.FormatFloat(float64(jFloat.Value), 'f', 8, 32))
}

type JInt struct {
	Value int
}

func (jInt JInt) ToJson(buffer *bytes.Buffer) {
	buffer.WriteString(strconv.FormatInt(int64(jInt.Value), 10))
}

type JBool struct {
	Value bool
}

func (jBool JBool) ToJson(buffer *bytes.Buffer) {
	if jBool.Value {
		buffer.WriteString("true")
	} else {
		buffer.WriteString("false")
	}
}

// Array
type JArray struct {
	Children []JElement
}

func (jArray JArray) ToJson(buffer *bytes.Buffer) {
	buffer.WriteRune('[')
	for i, child := range jArray.Children {
		if child == nil {
			buffer.WriteString("null")
		} else {
			child.ToJson(buffer)
		}

		if i < len(jArray.Children)-1 {
			buffer.WriteRune(',')
		}
	}
	buffer.WriteRune(']')
}

// Object
type JObject struct {
	Keys   []string
	Values []JElement
}

func (jObject JObject) ToJson(buffer *bytes.Buffer) {
	buffer.WriteRune('{')
	for i, key := range jObject.Keys {
		value := jObject.Values[i]
		if value != nil {
			buffer.WriteString("\"" + key + "\":")
			value.ToJson(buffer)
		} else {
			buffer.WriteString("\"" + key + "\":null")
		}

		if i < len(jObject.Keys)-1 {
			buffer.WriteRune(',')
		}
	}
	buffer.WriteRune('}')
}

type JDocument struct {
	Value          JElement
	tokensConsumed int
}

func (jDocument JDocument) ToJson(buffer *bytes.Buffer) {
	jDocument.Value.ToJson(buffer)
}

/*
Lexer Tokens
*/
type JToken struct {
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
	JTrueToken   JTokenType = 12
	JFalseToken  JTokenType = 13
)
