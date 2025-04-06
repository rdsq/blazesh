set _BLAZESH_DIR (dirname (realpath (status -f)))
set _BLAZESH_BIN_PATH $_BLAZESH_DIR/target/release/blazesh
set _BLAZESH_SHELL fish

if test -e $_BLAZESH_BIN_PATH
    function fish_prompt
        $_BLAZESH_BIN_PATH $status (count (jobs -p))
    end
else
    echo "blazesh: could not find the binary. You can compile it by using this command:" >&2
    echo "cd $_BLAZESH_DIR && cargo build --release" >&2
end
