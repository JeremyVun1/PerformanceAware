package jsonparser

import (
	. "haversine/jsonparser/jsonparse"
)

func Deserialize(fileChannel *chan []byte) JDocument {
	tokenChannel := make(chan []JToken)

	go Lex(fileChannel, &tokenChannel)
	return ParseTokens(&tokenChannel)
}

func Process(fileChannel *chan []byte) JDocument {
	Deserialize(fileChannel)

	return JDocument{}
}

/*
	improvements
	1. Stream the file
	2. channel and stream the lexer to the parser
	3. buffio instead of sprintf
	4. Remove comma token
	5. lexer to parse number instead of []rune
*/
