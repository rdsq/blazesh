_BLAZESH_DIR="${0:A:h}"

precmd() {
    export PS1="$($_BLAZESH_DIR/target/release/blazesh)"
}
