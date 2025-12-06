
## Spec

- Should be only limited by index sizes (and host memory)
- Should only use memory for modified content
- Should be able to keep track of modified content
- Should be fast
  - Using a simple HashMap is not enough

## Chunk size ?

### power of 2 ?

Yes, simplify calculous

### Which size

2x2, 4x4, 8x8 ? Too small ! 

16x16 ? 
- 256 * item size
- 2 level => grid of size 256*256, a bit small

32x32 ? 
- 1024 * item size
- 2 level => grid of size 1024*1024, better

64x64 ? 
- 4096 * item size, big for huge items
- 2 level => grid of size 4096*4096

128x128 ?
- 16384 * item size, too big for huge items
- 2 level => grid of size 16384*16384

256x256, ... ? Too big

### Conclusion

<!-- Smaller grid: 32x32 -->