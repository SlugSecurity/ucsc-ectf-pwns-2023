#!/usr/bin/env bash
CHAL=$(head -c 83 $1 | xxd -p -c 0)
CHAL=${CHAL^^}
echo $CHAL

RESP=$(echo -n $CHAL | nu --stdin -c 'let chal = $in; open replay.csv | where challenge == $chal | get response.0')
echo $RESP
echo -n $RESP | xxd -p -r - > $1