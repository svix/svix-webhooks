PYTHON := "python3.14"
HERE := justfile_directory()

# Print usage
default:
    @echo 'Most of the just commands are in the child directories (e.g., `server/`)'
    @echo 'You can invoke them with, e.g., `just server/lint`, or by cding to the relevant'
    @echo 'directory first'

# Regenerate all client libraries
[no-exit-message]
codegen:
    {{ PYTHON }} regen_openapi.py
