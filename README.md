# hubbard_ed
Hubbard ED in rust
# Install
## Dependencies
- rust
- cargo
- lapack

## Compile with cargo
- Build preset with all optimisations.
`cargo build --release`
- Build using specified data type profile. (ex. float32 and uint32)
`cargo build --release --features f32_u32 --no-default-features`
The executable will be in `./target/release/build/`
