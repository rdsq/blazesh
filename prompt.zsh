_BLAZESH_DIR="${0:A:h}"
_BLAZESH_BIN_PATH="$_BLAZESH_DIR/target/release/blazesh"

if [ -f "$_BLAZESH_BIN_PATH" ]; then
    precmd() {
        PS1="$($_BLAZESH_BIN_PATH "$?" "$(jobs | wc -l)")"
    }
else
    echo "blazesh: could not find the binary. You can compile it by using this command:" >&2
    echo "cd $_BLAZESH_DIR && cargo build --release" >&2
fi
