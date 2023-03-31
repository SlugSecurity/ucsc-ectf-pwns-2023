#!/usr/bin/env bash
echo -n UC > $1
sleep 0.1
echo -n Z > $1
sleep 0.1
xxd -r -p <(echo CC4538C6FAA9702D6D63D14B9063C3) > $1
sleep 0.1
xxd -r -p <(echo F26DC3F37CC8F78C39E7926CF8853A) > $1
sleep 0.1
xxd -r -p <(echo F5D2F5DACC03D20E7D35D2CD172FCF) > $1
sleep 0.1
xxd -r -p <(echo DD5794E727E766492376C765351E22) > $1
sleep 0.1
xxd -r -p <(echo 89698C9C7662666F7C50214819D2B3) > $1
sleep 0.1
xxd -r -p <(echo 6DEAB4B2000FD2CA718883DA779FCF) > $1
sleep 0.1
xxd -r -p <(echo 2FD0395AE0DC72C7DA3C57935A1B4F) > $1
sleep 0.1
xxd -r -p <(echo 0282D046B018D28D009EAC730AAE82) > $1
sleep 0.1
xxd -r -p <(echo 6FA73945ECA1380A) > $1