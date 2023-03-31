#!/usr/bin/env bash
echo -n UC > $1
sleep 0.1
echo -n Z > $1
sleep 0.1
xxd -r -p <(echo F5D29D6BE503BCE4CB22C5921B52C6) > $1
sleep 0.1
xxd -r -p <(echo 49FB31BEA1EE0779B7ABD758299560) > $1
sleep 0.1
xxd -r -p <(echo 03A8AFF96B436C6CFA1224C44F5A36) > $1
sleep 0.1
xxd -r -p <(echo 41149220AD4B30310D65247A246E11) > $1
sleep 0.1
xxd -r -p <(echo FD873305A642ADCD4542E77FDC25B0) > $1
sleep 0.1
xxd -r -p <(echo 8CD6C90D1A019A7FC94553355AC21C) > $1
sleep 0.1
xxd -r -p <(echo 372D9B6696B446AEDAB89A4A1AF8A0) > $1
sleep 0.1
xxd -r -p <(echo 00701433A7F998E4E312EA457F83E9) > $1
sleep 0.1
xxd -r -p <(echo 1DD170A0BA216095) > $1