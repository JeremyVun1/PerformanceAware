package jsonparse

import (
	"fmt"
	"strings"
	"unicode"
)

/*
{\"string\":\"value\",\"int\":5,\"float\":123.1231231233,\"null\":null,\"array\":[\"a\",\"b\"],\"obj\":{\"key\":\"value\"}}

	  :
	/   \
   str  str
*/

func Lex(jsonStr string) []jToken {
	jsonStr = strings.ReplaceAll(jsonStr, "\\", "")
	var tokens []jToken

	i := 0
	for i < len(jsonStr) {
		c := rune(jsonStr[i])

		if unicode.IsDigit(c) || c == '-' || c == '+' {
			token := parse_number(jsonStr[i:])
			i += len(token.Value)
			tokens = append(tokens, token)
		} else if c == '"' {
			token := parse_string(jsonStr[i:])
			i += len(token.Value) + 1
			tokens = append(tokens, token)
		} else if len(jsonStr[i:]) >= 4 && jsonStr[i:i+4] == "null" {
			tokens = append(tokens, jToken{Type: JNullToken})
			i += 4
		} else {
			switch c {
			case ' ':
				break
			case '\\':
				break
			case '{':
				tokens = append(tokens, jToken{Type: JStartObject})
			case '}':
				tokens = append(tokens, jToken{Type: JEndObject})
			case '[':
				tokens = append(tokens, jToken{Type: JStartArray})
			case ']':
				tokens = append(tokens, jToken{Type: JEndArray})
			case ':':
				tokens = append(tokens, jToken{Type: JColonToken})
			case ',':
				tokens = append(tokens, jToken{Type: JCommaToken})
			default:
				panic(fmt.Sprintf("Unsupported character %c", c))
			}
		}

		i += 1
	}

	return tokens
}

func parse_number(jsonStr string) jToken {
	var chars []rune

	i := 0
	c := rune(jsonStr[i])
	isFloat := false
	for unicode.IsDigit(c) || c == '-' || c == '+' || c == '.' {
		if c == '.' {
			isFloat = true
		}

		c = rune(jsonStr[i])
		chars = append(chars, c)
		i += 1
		c = rune(jsonStr[i])
	}

	if isFloat {
		return jToken{
			Type:  JFloatToken,
			Value: chars,
		}
	} else {
		return jToken{
			Type:  JIntToken,
			Value: chars,
		}
	}

}

func parse_string(jsonStr string) jToken {
	var chars []rune
	i := 0
	isFinished := false
	for i < len(jsonStr) {
		c := jsonStr[i]

		if c == '"' {
			if isFinished {
				return jToken{
					Type:  JStringToken,
					Value: chars,
				}
			} else {
				isFinished = true
			}
		} else if c != '\\' {
			chars = append(chars, rune(c))
		}

		i += 1
	}

	panic(fmt.Sprintf("Invalid string at %c '%s'", i, jsonStr))
}
