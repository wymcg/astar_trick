# A* Visualizer Trick
A [Matricks](https://github.com/wymcg/matricks) plugin that generates a maze and then solves it with A*.

![astartrick](https://github.com/wymcg/astar_trick/assets/3410869/97fcfc80-17a8-483a-84e6-17c09849afad)

## Build
- Install the `wasm32-wasi` toolchain by running `rustup target add wasm32-wasi`
- Run `cargo build --release --target wasm32-wasi`
- Run the plugin with [Matricks](https://github.com/wymcg/matricks) (on a Raspberry Pi) or with [Simtricks](https://github.com/wymcg/simtricks) (on other devices).
