
1. Generate 10,000,000 pairs

> go run . -generate -size=10000000 -f="test.json"

2. Processing haversine pairs

> go run. -process -f="test.json"

3. To test parsing,

> go run . -parse -f="test.json" -output