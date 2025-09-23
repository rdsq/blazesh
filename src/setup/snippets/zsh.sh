_blazesh_shell=zsh
if [[ ! "${precmd_functions}" =~ _blazesh_construct_prompt ]]; then
    precmd_functions+=(_blazesh_construct_prompt)
fi
