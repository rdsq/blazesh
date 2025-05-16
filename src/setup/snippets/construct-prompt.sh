_blazesh_construct_prompt () {
    local exit_code="$?"
    local jobs="$(jobs -p | grep '^\[' | wc -l)"
    PS1=$($_BLAZESH_BIN_PATH "$exit_code" "$jobs")
}
