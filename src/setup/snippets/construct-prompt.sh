_blazesh_construct_prompt () {
    local exit_code="$?"
    local jobs="$(jobs -p | grep '^\[' | wc -l)"
    PS1=$(blazesh prompt -- "$exit_code" "$jobs" "$_blazesh_shell")
}
