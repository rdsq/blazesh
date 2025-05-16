set _BLAZESH_SHELL fish

function fish_prompt
    blazesh prompt $status (count (jobs -p))
end
