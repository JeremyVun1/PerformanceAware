package jsonparse

import (
	"fmt"
	"haversine/json-parser/util"
	"strconv"
	"strings"
)

func Deserialize(jsonStr string) JDocument {
	jsonStr = strings.ReplaceAll(jsonStr, " ", "")
	jsonStr = strings.ReplaceAll(jsonStr, "\r\n", "")

	fmt.Printf("Deserializing: %s\n", jsonStr)

	tokens := Lex(jsonStr)

	return parse_document(tokens)
}

func parse_document(tokens []jToken) JDocument {
	switch tokens[0].Type {
	case JStartArray:
		return JDocument{Value: parse_array(tokens)}
	case JStartObject:
		return JDocument{Value: parse_object(tokens)}
	default:
		panic(fmt.Sprintf("JsonDocument cannot hold type %d", tokens[0].Type))
	}
}

func parse_array(tokens []jToken) JArray {
	if len(tokens) < 2 {
		panic(fmt.Sprintf("Cannot parse array from %d tokens", len(tokens)))
	}
	if tokens[0].Type != JStartArray {
		panic("JArray must start with a [ token")
	}

	result := JArray{}

	tokens = tokens[1:]
	i := 0
	for i < len(tokens) {
		token := tokens[i]

		switch token.Type {
		case JStringToken:
			val := string(token.Value)
			result.Children = append(result.Children, JString{Value: val})
			i += 1
		case JFloatToken:
			val, err := strconv.ParseFloat(string(token.Value), 32)
			util.Check(err)
			result.Children = append(result.Children, JFloat{Value: float32(val)})
			i += 1
		case JIntToken:
			val, err := strconv.ParseInt(string(token.Value), 10, 32)
			util.Check(err)
			result.Children = append(result.Children, JInt{Value: int(val)})
			i += 1
		case JNullToken:
			result.Children = append(result.Children, nil)
			i += 1
		case JCommaToken:
			i += 1
		case JStartObject:
			var token = parse_object(tokens[i:])
			result.Children = append(result.Children, token)
			i += token.size
		case JStartArray:
			var token = parse_array(tokens[i:])
			result.Children = append(result.Children, token)
			i += token.size
		case JEndArray:
			result.size = i + 2 // start & end tokens
			return result
		default:
			panic(fmt.Sprintf("JArray does not support type %d", token.Type))
		}
	}

	return result
}

func parse_object(tokens []jToken) JObject {
	if len(tokens) < 2 {
		panic(fmt.Sprintf("Cannot parse object from %d tokens", len(tokens)))
	}
	if tokens[0].Type != JStartObject {
		panic("JObject must start with a '{' token")
	}

	result := JObject{}
	tokens = tokens[1:]

	i := 0
	for i < len(tokens) {
		token := tokens[i]

		switch token.Type {
		case JStringToken:
			key, value := parse_kv(tokens[i:])
			result.Keys = append(result.Keys, key.Value)
			result.Values = append(result.Values, value)
			if value == nil {
				i += 2 + 1
			} else {
				i += 2 + value.getSize()
			}
		case JCommaToken:
			i += 1
		case JEndObject:
			result.size = i + 2 // start & end tokens
			return result
		default:
			panic(fmt.Sprintf("JObject does not support type %d", token.Type))
		}
	}

	return result
}

func parse_kv(tokens []jToken) (JString, JToken) {
	if len(tokens) < 3 {
		panic(fmt.Sprintf("Cannot parse key value from %d tokens", len(tokens)))
	}
	if tokens[1].Type != JColonToken {
		panic("missing : token")
	}
	if tokens[0].Type != JStringToken {
		panic("key must be a string token")
	}

	key := JString{Value: string(tokens[0].Value)}

	switch tokens[2].Type {
	case JStringToken:
		return key, JString{Value: string(tokens[2].Value)}
	case JFloatToken:
		val, err := strconv.ParseFloat(string(tokens[2].Value), 32)
		util.Check(err)
		return key, JFloat{Value: float32(val)}
	case JIntToken:
		val, err := strconv.ParseInt(string(tokens[2].Value), 10, 32)
		util.Check(err)
		return key, JInt{Value: int(val)}
	case JNullToken:
		return key, nil
	case JStartObject:
		return key, parse_object(tokens[2:])
	case JStartArray:
		return key, parse_array(tokens[2:])
	default:
		panic(fmt.Sprintf("Unsupported value type %d for key %s", tokens[2].Type, key.Value))
	}
}

func print_tokens(tokens []jToken) {
	for _, token := range tokens {
		fmt.Printf("{ type: %d}", token.Type)
	}
	fmt.Print("\n")
}

/*
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
*/
