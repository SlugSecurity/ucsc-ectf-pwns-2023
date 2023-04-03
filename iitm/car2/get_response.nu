#!/usr/bin/env nu

def main [serial: string, esp32: string] {
    mut a = open replay.csv
    $a | each { |i|
        echo "l01\n" out> $esp32
        echo "h01\n" out> $esp32
        $i.challenge out> $serial
        $i.response = (head -c 57 $serial | xxd -p -c 0 | str upcase)
        sleep 100ms
    }
}