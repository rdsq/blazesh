if [ -n "$BASH_VERSION" ]; then
    eval "$(blazesh setup bash)"
elif [ -n "$ZSH_VERSION" ]; then
    eval "$(blazesh setup zsh)"
else
    echo "blazesh: unknown shell" >&2
fi
