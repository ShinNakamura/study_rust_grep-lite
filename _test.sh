#! /bin/bash
# 簡易テスト
unalias -a

f=./test/input/dates.txt
test -f $f || exit 1

result=$(cargo run -- '^\d+.+:' $f)
if test "_$result" = "_2022-08-08T20:41:35+09:00"
then
    echo "Ok"
else
    echo "Failed"
fi

result=$(cat $f | cargo run -- '^\d+.+:')
if test "_$result" = "_2022-08-08T20:41:35+09:00"
then
    echo "Ok"
else
    echo "Failed"
fi

result=$(cat $f | cargo run -- -o 'JST \d+')
if test "_$result" = "_JST 2022"
then
    echo "Ok"
else
    echo "Failed"
fi
