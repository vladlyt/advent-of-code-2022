#!/bin/bash

DAY_NUMBER="${1}"

NEW_FOLDER="day-$DAY_NUMBER"

cp -r base/ "$NEW_FOLDER/"

sed -i '' "s/day_01/day_$DAY_NUMBER/g" "$NEW_FOLDER/src/bin/part-1.rs"
sed -i '' "s/day_01/day_$DAY_NUMBER/g" "$NEW_FOLDER/src/bin/part-2.rs"
sed -i '' "s/day-01/day-$DAY_NUMBER/g" "$NEW_FOLDER/Cargo.toml"