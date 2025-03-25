# The_International_Rust

## Upload

### MMO

`npm run deploy -- --server jaysee --release --`

## Quickstart:

```
# Install rustup: https://rustup.rs/

# Install wasm-pack
cargo install wasm-pack

# Install wasm-opt
cargo install wasm-opt

# Install Node.js for build steps - versions 16 through 22 have been tested, any should work
# nvm is recommended but not required to manage the install, follow instructions at:
# Mac/Linux: https://github.com/nvm-sh/nvm
# Windows: https://github.com/coreybutler/nvm-windows

# Installs Node.js at version 20
# (all versions within LTS support should work;
# 20 is recommended due to some observed problems on Windows systems using 22)
nvm install 20
nvm use 20

# Clone the starter
git clone https://github.com/rustyscreeps/screeps-starter-rust.git
cd screeps-starter-rust
# note: if you customize the name of the crate, you'll need to update the MODULE_NAME
# variable in the js_src/main.js file and the module import with the updated name, as well
# as the "name" in the package.json

# Install dependencies for JS build
npm install

# Copy the example config, and set up at least one deployment mode.
cp .example-screeps.yaml .screeps.yaml
nano .screeps.yaml

# compile for a configured server but don't upload
npm run deploy -- --server ptr --dryrun

# compile and upload to a configured server
npm run deploy -- --server mmo
```