set quiet := true

HERE := justfile_directory()

# The first recipe runs when you invoke `just` without args.
_default:
    just --list --justfile {{ justfile() }}

current_version := read(HERE / ".version")

# Prepare a new release version, bumping the current version by the specified level

# (which should be one of "major", "minor", or "patch")
[no-exit-message]
prepare-release level:
    git fetch -q
    if [[ -n `git status --porcelain --untracked=no` ]]; then echo >&2 "{{ style('error') }}working directory is not clean{{ NORMAL }}" ; exit 1 ; fi
    git checkout -b $USER/oss-release-`date +%Y%m%d`
    yarn install --silent
    @echo "{{ CYAN }}Bumping version at level {{ level }} {{ NORMAL }}"
    yarn bump-version `yarn run --silent semver -i {{ level }} {{ current_version }}`
    cd svix-cli && cargo build
    cd server && cargo build
    cd bridge && cargo build
    cd rust && cargo build
    pre-commit run --all
    {{ if env('EDITOR') != "" { env('EDITOR') } else { "nano" } }} ChangeLog.md
    git commit -am "Publish version $(cat .version)"
    @echo "{{ BOLD }}{{ CYAN }}Version bumped to `cat .version`{{ NORMAL }}"
