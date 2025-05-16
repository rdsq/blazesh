export _BLAZESH_SHELL=bash
if [ ! -n "$PROMPT_COMMAND" ]; then
    PROMPT_COMMAND="_blazesh_construct_prompt"
elif [[ "$PROMPT_COMMAND" != *"_blazesh_construct_prompt"* ]]; then
    PROMPT_COMMAND="$PROMPT_COMMAND; _blazesh_construct_prompt"
fi
