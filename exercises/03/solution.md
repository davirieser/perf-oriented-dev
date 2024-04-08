
# A

## Benchmark Results

All functions were included whose time percentage was higher than 0.5 on average.
The benchmarks were run for all of the variants.
The results of the _s and _w variants are not included as they are too unreliable because of their small sample size/short execution time.

### Local Results

| Function Name | Mean Percentage | Calls |
|--|--|--|
| binvchrs | 33% | 100 Million to Billion |
| matmul_sub | 15.5% | 100 Million to Billion |
| y_solve | 12% | 201 |
| z_solve | 11.5% | 201 |
| x_solve | 9.75% | 201 |
| matvec_sub | 8.6% | 100 Million to Billion |
| compute_rhs | 7.7% | 202 |
| add | 0.4% | 201 |

### LCC3 Results

| Function Name | Mean Percentage | Calls |
|--|--|--|
| binvchrs | 30% | 100 Million to Billion |
| matmul_sub | 18% | 100 Million to Billion |
| z_solve | 13% | 201 |
| y_solve | 13% | 201 |
| x_solve | 11% | 201 |
| compute_rhs | 8.75% | 202 |
| matvec_sub | 5% | 100 Million to Billion |
| add | 0.5% | 201 |

## Findings

All of the important functions scale linearly with the problem size.
The most important functions are `binvcrhs` and `matmul_sub` as they are both called Millions to billions of times. 
The `x_solve`, `y_solve` and `z_solve` are the next most vital functions.
They each call `binvcrhs`, `matmul_sub` and `matvec_sub` millions of times.
I think the `x_solve`, `y_solve` and `z_solve` functions don't have much overhead and spend most of their time in the previously named subfunctions, but i am not sure if I am reading the call tree correctly. 
`compute_rhs` and `add` are also important as they do not rely on any other functions and can thus probably be optimized more easily.

#### For small sample sizes

When using the `_s` variant, all of the computation time is evenly spent across  `matmul_sub`, `binvrhs`, and `z_solve`.
With the `_w` variant the numbers become similar to the bigger variants but still too unreliable to use.




