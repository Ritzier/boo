# Compile Rust code
cargo build --release

# Create `lua/`, neovim would read the file in `lua/`
mkdir -p lua

# Check if lua/boo.so exists and remove it if it does
if [ -f lua/boo.so ]; then
    rm lua/boo.so
fi

# Link target/release/libboo.so to lua/boo.so
ln -sfn ../target/release/libboo.so lua/boo.so

