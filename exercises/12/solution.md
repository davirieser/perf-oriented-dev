
### A) Setup and Basic Execution

##### GCC - No Jump Table

```
100      x fibonacci_naive(30) time:  13.0095 s  --  832040
10000000 x fibonacci_tail(30)  time:  12.8127 s  --  832040
25000000 x fibonacci_iter(30)  time:  11.7029 s  --  832040
```

##### GCC - With Jump Table

```
100      x fibonacci_naive(30) time:  12.9171 s  --  832040
10000000 x fibonacci_tail(30)  time:  12.6853 s  --  832040
25000000 x fibonacci_iter(30)  time:  10.9147 s  --  832040
```

##### Clang - No Jump Table

```
100      x fibonacci_naive(30) time:  14.4256 s  --  832040
10000000 x fibonacci_tail(30)  time:  13.9045 s  --  832040
25000000 x fibonacci_iter(30)  time:  12.8259 s  --  832040
```

##### Clang - With Jump Table

```
100      x fibonacci_naive(30) time:  14.1838 s  --  832040
10000000 x fibonacci_tail(30)  time:  13.5195 s  --  832040
25000000 x fibonacci_iter(30)  time:  12.8535 s  --  832040
```

##### GCC 8 - No Jump Table 

```
100 x fibonacci_naive(30)     time:  13.3761 s  --  832040
10000000 x fibonacci_tail(30) time:  12.3656 s  --  832040
25000000 x fibonacci_iter(30) time:  11.8834 s  --  832040
```

##### GCC 8 - With Jump Table 

```
100 x fibonacci_naive(30)     time:  13.2953 s  --  832040
10000000 x fibonacci_tail(30) time:  12.2686 s  --  832040
25000000 x fibonacci_iter(30) time:  12.0594 s  --  832040
```

### B) Profiling



### C) Code Understanding

- Major Phases of Lua Interpreter:
  - Command Line Argument Parsing `lua.c` or `luac.c` 
  - Lexing `llex.c`
  - Parsing `lparser.c`
  - Execution `lua_pcallk` in `lua.c`

- LUA_USE_JUMPTABLE is a Compile Flag that is enabled in GCC by default.

  If it is set it imports the `ljumptab.h` on line 1153 of `lvm.c`.

  This import converts the giant switch case in `lvm.c` on line 1183 to the 1896 into a giant label table and `goto` statements by redefining the `vmdispatch` and `vmcase` macros.

### D) Optimization
