_blazesh_construct_prompt () {
    local exit_code="$?"
    local jobs="$(jobs -p | grep '^\[' | wc -l)"
    PS1=$($_BLAZESH_BIN_PATH "$exit_code" "$jobs")
}

if [ -n "$BASH_VERSION" ]; then
    export _BLAZESH_DIR="$(dirname "${BASH_SOURCE[0]}")"
    export _BLAZESH_SHELL=bash
elif [ -n "$ZSH_VERSION" ]; then
    export _BLAZESH_DIR="${0:A:h}"
    export _BLAZESH_SHELL=zsh
else
    echo "blazesh: unknown shell" >&2
    return 1
fi
if [ ! -n "$_BLAZESH_BIN_PATH" ]; then
    # allow to set custom bin path
    export _BLAZESH_BIN_PATH="$_BLAZESH_DIR/target/release/blazesh"
fi

if [ -f "$_BLAZESH_BIN_PATH" ]; then
    if [ -n "$BASH_VERSION" ]; then
        if [ ! -n "$PROMPT_COMMAND" ]; then
            PROMPT_COMMAND="_blazesh_construct_prompt"
        elif [[ "$PROMPT_COMMAND" != *"_blazesh_construct_prompt"* ]]; then
            PROMPT_COMMAND="$PROMPT_COMMAND; _blazesh_construct_prompt"
        fi
    elif [ -n "$ZSH_VERSION" ] && [[ ! "${precmd_functions}" =~ _blazesh_construct_prompt ]]; then
        precmd_functions+=(_blazesh_construct_prompt)
    fi
else
    echo "blazesh: could not find the binary. You can compile it by using this command:" >&2
    echo "cd $_BLAZESH_DIR && cargo build --release" >&2
fi
