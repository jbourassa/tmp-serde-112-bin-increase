# Binary size increase w/ serde_json 1.0.111 vs 1.0.112

Repro repo that compiles a minimal Wasm binary compiled with 1.0.111 vs 1.0.112.


**Prereq**:
- cargo-wasi: `cargo install cargo-wasi`
- optionally `wasm-tools`: `cargo install wasm-tools`



**Build & useful commands**:
- Build both versions: `(cd serde_json-111 && cargo wasi build --release) && (cd serde_json-112 && cargo wasi build --release)`
- Filesize: `find . -name serde-float.wasm | xargs du -h`
- Copy the WAT and Wasm so they're checked-in:
    ```
    cp ./serde_json-111/target/wasm32-wasi/release/serde-float.wasm serde-111.wasm &&
      cp ./serde_json-112/target/wasm32-wasi/release/serde-float.wasm serde-112.wasm &&
      wasm2wat serde-111.wasm > serde-111.wat && 
      wasm2wat serde-112.wasm > serde-112.wat
    ```

**Observations**:
- 111 is 83kb, 112 is 92kb
- 111 has more instructions, but a much smaller DATA segment
