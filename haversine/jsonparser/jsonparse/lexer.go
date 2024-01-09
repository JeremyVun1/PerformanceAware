package jsonparse

import (
	"fmt"
	"unicode"
)

func Lex(inChannel *chan []byte, outChannel *chan []JToken) {
	for {
		data, status := <-*inChannel
		if len(data) == 0 && !status {
			close(*outChannel)
			break
		}

		*outChannel <- lex(data)
	}
}

func lex(data []byte) []JToken {
	tokens := make([]JToken, 0, len(data))

	i := 0
	for i < len(data) {
		c := rune(data[i])

		if c == 't' {
			tokens = append(tokens, JToken{Type: JTrueToken})
			i += 3
		} else if c == 'f' {
			tokens = append(tokens, JToken{Type: JFalseToken})
			i += 4
		} else if c == 'n' {
			tokens = append(tokens, JToken{Type: JNullToken})
			i += 3
		} else if unicode.IsDigit(c) || c == '-' || c == '+' {
			token := parse_number(data[i:])
			i += len(token.Value) - 1
			tokens = append(tokens, token)
		} else if c == '"' {
			token := parse_string(data[i:])
			i += len(token.Value) + 1
			tokens = append(tokens, token)
		} else {
			switch c {
			case '\n':
			case ' ':
			case '\\':
				break
			case '{':
				tokens = append(tokens, JToken{Type: JStartObject})
			case '}':
				tokens = append(tokens, JToken{Type: JEndObject})
			case '[':
				tokens = append(tokens, JToken{Type: JStartArray})
			case ']':
				tokens = append(tokens, JToken{Type: JEndArray})
			case ':':
				tokens = append(tokens, JToken{Type: JColonToken})
			case ',':
				tokens = append(tokens, JToken{Type: JCommaToken})
			default:
				panic(fmt.Sprintf("Unsupported character '%c'", c))
			}
		}

		i += 1
	}

	return tokens
}

func parse_number(data []byte) JToken {
	var chars []rune

	i := 0
	c := rune(data[i])
	isFloat := false
	for unicode.IsDigit(c) || c == '-' || c == '+' || c == '.' {
		if c == '.' {
			isFloat = true
		}

		c = rune(data[i])
		chars = append(chars, c)
		i += 1
		c = rune(data[i])
	}

	if isFloat {
		return JToken{
			Type:  JFloatToken,
			Value: chars,
		}
	} else {
		return JToken{
			Type:  JIntToken,
			Value: chars,
		}
	}

}

func parse_string(data []byte) JToken {
	var chars []rune
	i := 0
	isFinished := false
	for i < len(data) {
		c := data[i]

		if c == '"' {
			if isFinished {
				return JToken{
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

	panic(fmt.Sprintf("Invalid string at %c '%s'", i, data))
}
