# How to use

For BlueOS (red-pill)

```shell
# Prepare the environment with cmake and cargo
sudo apt-get update && sudo apt-get install cmake git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Clone and move to Cpp folder
git clone https://github.com/bluerobotics/navigator-lib.git
cd examples/cpp

# Build with the following commands
cmake -B build -DCMAKE_BUILD_TYPE=Debug && cmake --build build --config Debug --parallel

# Run the binary
./build/simple

```
