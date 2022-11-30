cargo new day-$1
cd day-$1
curl --cookie "session=$(cat ../.secret-cookie)" https://adventofcode.com/2022/day/$1/input > input.txt
cp ../template.rs src/main.rs
