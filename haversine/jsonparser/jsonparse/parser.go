package jsonparse

import (
	"fmt"
	"haversine/jsonparser/util"
	"strconv"
)

func ParseTokens(tokenChannel *chan []JToken) JDocument {
	emitter := NewTokenEmitter(tokenChannel)

	for {
		token, _ := emitter.Next()
		switch token.Type {
		case JStartArray:
			return JDocument{Value: parse_array(emitter), tokensConsumed: emitter.count}
		case JStartObject:
			return JDocument{Value: parse_object(emitter), tokensConsumed: emitter.count}
		default:
			panic(fmt.Sprintf("JsonDocument cannot hold type %d", token.Type))
		}
	}
}

func parse_object(emitter *TokenEmitter) JObject {
	result := JObject{}

	token, _ := emitter.Next()
	for token.Type != JEndObject {
		switch token.Type {
		case JStringToken:
			key := JString{Value: string(token.Value)}
			value := parse_element(emitter)
			result.Keys = append(result.Keys, key.Value)
			result.Values = append(result.Values, value)
		case JCommaToken:
			break
		default:
			panic(fmt.Sprintf("JObject does not support type %d", token.Type))
		}

		nextToken, _ := emitter.Next()
		token = nextToken
	}

	return result
}

func parse_array(emitter *TokenEmitter) JArray {
	result := JArray{}

	token, _ := emitter.Next()
	for token.Type != JEndArray {
		switch token.Type {
		case JStringToken:
			val := string(token.Value)
			result.Children = append(result.Children, JString{Value: val})
		case JTrueToken:
			result.Children = append(result.Children, JBool{Value: true})
		case JFalseToken:
			result.Children = append(result.Children, JBool{Value: false})
		case JFloatToken:
			val, err := strconv.ParseFloat(string(token.Value), 32)
			util.Check(err)
			result.Children = append(result.Children, JFloat{Value: float32(val)})
		case JIntToken:
			val, err := strconv.ParseInt(string(token.Value), 10, 32)
			util.Check(err)
			result.Children = append(result.Children, JInt{Value: int(val)})
		case JNullToken:
			result.Children = append(result.Children, nil)
		case JCommaToken:
			break
		case JStartObject:
			var token = parse_object(emitter)
			result.Children = append(result.Children, token)
		case JStartArray:
			var token = parse_array(emitter)
			result.Children = append(result.Children, token)
		default:
			panic(fmt.Sprintf("JArray does not support type %d", token.Type))
		}

		nextToken, _ := emitter.Next()
		token = nextToken
	}

	return result
}

func parse_element(emitter *TokenEmitter) JElement {
	colonToken, _ := emitter.Next()
	if colonToken.Type != JColonToken {
		panic(fmt.Sprintf("Expected colon token but got <%d>", colonToken.Type))
	}

	valueToken, _ := emitter.Next()

	switch valueToken.Type {
	case JStringToken:
		return JString{Value: string(valueToken.Value)}
	case JFloatToken:
		val, err := strconv.ParseFloat(string(valueToken.Value), 32)
		util.Check(err)
		return JFloat{Value: float32(val)}
	case JIntToken:
		val, err := strconv.ParseInt(string(valueToken.Value), 10, 32)
		util.Check(err)
		return JInt{Value: int(val)}
	case JNullToken:
		return nil
	case JStartObject:
		return parse_object(emitter)
	case JStartArray:
		return parse_array(emitter)
	default:
		panic(fmt.Sprintf("Unexpected value type while parsing element %d", valueToken.Type))
	}
}

func print_tokens(tokens []JToken) {
	for _, token := range tokens {
		fmt.Printf("{ type: %d, value: %s}", token.Type, string(token.Value))
	}
	fmt.Print("\n")
}
