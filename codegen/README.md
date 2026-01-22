# Client SDK code generation

This directory contains code to generate the SDKs (+ Svix CLI).
Here is a quick overview for how to use it:

## Updating the SDKs

To regenerate the code after updating `lib-openapi.json`, run `regen_openapi.py` from the repository root.

## openapi-codegen

To update the underlying codegen tool `openapi-codegen`, update `OPENAPI_CODEGEN_IMAGE` in the same file.

Every push to its upstream repo's main branch results in a new image being built.
You can find those on https://github.com/svix/openapi-codegen/pkgs/container/openapi-codegen.

You can also build it locally by cloning the repo and running `podman build . -t openapi-codegen`.
In that case, use `localhost/openapi-codegen` for the `OPENAPI_CODEGEN_IMAGE`.

## Updating or adding templates

To add a SDK in a new language or modify the generated code for one,
look at `codegen.toml` and `templates/`. Every SDK generally has

- one template that renders every individual API resource (`api_resource.XY.jinja`),
- one template that renders every individual component type (`component_type.XY.jinja`),
- one or more templates that coalesce the submodules into a parent module (`*summary.XY.jinja`).

The parent directory of the template being used is used as the include directory, so templates
can `{% include %}` files from the same directory or subdirectories.
