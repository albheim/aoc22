#!/usr/bin/bash

echo "Building all"
cargo build --release

ALL_DAYS=$(ls target/release | grep day[0-9].\$)

individual_sum=0
echo "Running individual"
for d in $ALL_DAYS ; do
    start_time="$(date -u +%s.%N)"
    ./target/release/$d a >> /dev/null
    mid_time="$(date -u +%s.%N)"
    ./target/release/$d b >> /dev/null
    end_time="$(date -u +%s.%N)"
    elapseda="$(bc <<<"$mid_time-$start_time")"
    elapsedb="$(bc <<<"$end_time-$mid_time")"
    individual_sum="$(bc <<<"$individual_sum+$elapseda+$elapsedb")"
    echo "$(basename $d)a: $elapseda seconds"
    echo "$(basename $d)b: $elapsedb seconds"
done
echo "   Sum: $individual_sum seconds"

echo ""
echo "Running all"
start_time="$(date -u +%s.%N)"
for d in $ALL_DAYS ; do
    ./target/release/$d a >> /dev/null
    ./target/release/$d b >> /dev/null
done
end_time="$(date -u +%s.%N)"

elapsed="$(bc <<<"$end_time-$start_time")"
echo " Total: $elapsed seconds"