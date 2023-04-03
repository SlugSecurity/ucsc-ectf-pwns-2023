#!/usr/bin/env dash
echo challenge,response > replay.csv
for x in $(cat challenges.txt); do
    KEY=0000
    while [ $KEY != "aa0a" ]; do
        echo "l0r" > $2
        echo "h0r" > $2
        sleep 0.1
        echo "l01" > $2
        echo "h01" > $2
        KEY=$(head -c 2 $1 | xxd -p -c 0)
        echo $KEY
    done
    echo $x | xxd -p -r - > $1
    echo -n $x, >> replay.csv
    head -c 57 $1 | xxd -p -c 0 >> replay.csv
    echo >> replay.csv
done