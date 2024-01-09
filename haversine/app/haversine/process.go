package haversine

import (
	"fmt"
	. "haversine/jsonparser/jsonparse"
	"math"
	"time"
)

func Process(jsonDoc *JDocument) {
	root := jsonDoc.Value.(JObject)
	pairs := GetItem[JArray](&root, "pairs")

	start := time.Now()
	pairs.Children = pairs.Children[:10000]

	var result float64 = 0
	var mod float64 = float64(1) / float64(len(pairs.Children))
	for _, child := range pairs.Children {
		c := child.(JObject)
		x0 := float64(GetItem[JFloat](&c, "x0").Value)
		x1 := float64(GetItem[JFloat](&c, "x1").Value)
		y0 := float64(GetItem[JFloat](&c, "y0").Value)
		y1 := float64(GetItem[JFloat](&c, "y1").Value)

		result += mod * referenceHaversine(x0, y0, x1, y1, 6372.8)
	}

	fmt.Printf("Finished processing haversines %v result: %f \n", time.Since(start), result)
}

func square(a float64) float64 {
	result := (a * a)
	return result
}

func radiansFromDegrees(degrees float64) float64 {
	result := 0.01745329251994329577 * degrees
	return result
}

// NOTE(casey): EarthRadius is generally expected to be 6372.8
func referenceHaversine(X0 float64, Y0 float64, X1 float64, Y1 float64, EarthRadius float64) float64 {
	/* NOTE(casey): This is not meant to be a "good" way to calculate the Haversine distance.
	   Instead, it attempts to follow, as closely as possible, the formula used in the real-world
	   question on which these homework exercises are loosely based.
	*/

	lat1 := Y0
	lat2 := Y1
	lon1 := X0
	lon2 := X1

	dLat := radiansFromDegrees(lat2 - lat1)
	dLon := radiansFromDegrees(lon2 - lon1)
	lat1 = radiansFromDegrees(lat1)
	lat2 = radiansFromDegrees(lat2)

	a := square(math.Sin(dLat/2.0)) + math.Cos(lat1)*math.Cos(lat2)*square(math.Sin(dLon/2))
	c := 2.0 * math.Asin(math.Sqrt(a))

	result := EarthRadius * c
	return result
}
