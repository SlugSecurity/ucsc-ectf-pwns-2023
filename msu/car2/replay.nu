#!/usr/bin/env nu

def main [
    --serial: string
] {
    let a = open replay.csv
    let chal = (head -c 65 $serial | xxd -p -c 0 | str upcase)
    echo $chal
    let resp = ($a | where challenge == $chal | get response.0)
    echo $resp
    # $resp | xxd -p -r - out> $serial
}