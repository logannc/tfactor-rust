: fib ( n -- n ) dup 1 < [ drop 1 ] [ dup 1 - fib swap 2 - fib + ] if ;

10 fib .