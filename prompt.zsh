export _BLAZESH_DIR="${0:A:h}"
export _BLAZESH_BIN_PATH="$_BLAZESH_DIR/target/release/blazesh"

if [ -f "$_BLAZESH_BIN_PATH" ]; then
    precmd() {
        local exit_code="$?"
        local jobs="$(jobs | wc -l)"
        PS1=$($_BLAZESH_BIN_PATH "$exit_code" "$jobs")
    }
else
    echo "blazesh: could not find the binary. You can compile it by using this command:" >&2
    echo "cd $_BLAZESH_DIR && cargo build --release" >&2
fi
