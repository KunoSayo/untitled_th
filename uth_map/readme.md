# Untitled touhou map ... lib (?)

# File Format Information (FFI (X))

| Bytes | Usage  | Content/Type |
|-------|--------|--------------|
| 3     | Header | uth          |
| 4     | blocks | u32 be       |
| 4     | Width  | u32 be       |
| 4     | Height | u32 be       |

And for every block

| Bytes     | Usage             | Content/Type    |
|-----------|-------------------|-----------------|
| 1         | Flag Count        | u8              |
| 1         | Key Value Count   | u8              |
| 1         | Bounding          | u8              |
| Not Fixed | Block Resource Id | zero-end string |

And for every flag contains zero-end string, indicated that the block has the flags.  
For every key-value, contains zero-end string and following 4bytes b-encoded f32.


| Bytes              | Usage         | Content/Type                                   |
|--------------------|---------------|------------------------------------------------|
| 4 * Width * Height | Block in slot | the block (index start at 1 and 0 for no block |