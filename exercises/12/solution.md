
### A) Setup and Basic Execution

```
100      x fibonacci_naive(30)     time:  14.2116 s  --  832040
10000000 x fibonacci_tail(30)      time:  12.2245 s  --  832040
25000000 x fibonacci_iter(30)      time:  12.0391 s  --  832040
```

### B) Profiling



### C) Code Understanding

- LUA_USE_JUMPTABLE is a Compile Flag that is enabled in GCC by default.
  If it is set it imports the `ljumptab.h` on line 1153 of `lvm.c`.
  This import converts the giant switch case in `lvm.c` on line 1183 to the 1896 into a giant label table and `goto` statements by redefining the `vmdispatch` and `vmcase` macros.

### D) Optimization
