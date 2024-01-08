package jsonparse

import "fmt"

func GetItem[T any](jObject JObject, key string) T {
	for i := range jObject.Keys {
		if jObject.Keys[i] == key {
			return jObject.Values[i].(T)
		}
	}
	panic(fmt.Sprintf("key: %s not found", key))
}

func GetItemAt[T any](jArray JArray, i int) T {
	return jArray.Children[i].(T)
}
