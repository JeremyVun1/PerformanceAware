package jsonparse

type TokenEmitter struct {
	source *chan []JToken
	buffer []JToken
	i      int
	isDone bool
	count  int
}

func NewTokenEmitter(source *chan []JToken) *TokenEmitter {
	result := new(TokenEmitter)
	result.source = source
	result.buffer = <-*source
	result.isDone = false
	result.i = 0
	result.count = 0

	return result
}

func (emitter *TokenEmitter) Peek() (JToken, bool) {
	if emitter.isDone {
		return JToken{Type: JNullToken}, false
	}

	// refresh the buffer if we ran out of tokens
	if emitter.i >= len(emitter.buffer) {
		emitter.i = 0
		data, state := <-*emitter.source
		if !state {
			emitter.isDone = true
		}

		emitter.buffer = data
	}

	return emitter.buffer[emitter.i], true
}

func (emitter *TokenEmitter) Next() (JToken, bool) {
	token, status := emitter.Peek()
	emitter.i += 1
	emitter.count += 1
	return token, status
}
