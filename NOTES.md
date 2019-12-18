## Notes

### Operator Abuse
```rust
// Sorta makes sense
cpu |= [0x01, 0, 0, 0]
cpu &= [0x01, 0, 0, 0]
cpu += [0x01, 0, 0, 0]

// Looks cool
cpu <<= [0x01, 0, 0, 0]

// Other
cpu ^= [0x01, 0, 0, 0]
cpu >>= [0x01, 0, 0, 0]
cpu -= [0x01, 0, 0, 0]
cpu *= [0x01, 0, 0, 0]
cpu /= [0x01, 0, 0, 0]
cpu %= [0x01, 0, 0, 0]
```