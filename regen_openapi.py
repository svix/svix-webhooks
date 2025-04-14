#!/usr/bin/env python3
import json
import os
import pathlib
import random
import shutil
import string
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

try:
    import tomllib
except ImportError:
    print("Python 3.11 or greater is required to run the codegen")
    exit(1)

OPENAPI_CODEGEN_IMAGE = "ghcr.io/svix/openapi-codegen:20250414-293"
REPO_ROOT = pathlib.Path(__file__).parent.resolve()
DEBUG = os.getenv("DEBUG") is not None
GREEN = "\033[92m"
BLUE = "\033[94m"
CYAN = "\033[96m"
ENDC = "\033[0m"


# I can't sys.exit from a thread
class ExitException(Exception):
    def __init__(self, code: int):
        super().__init__("")
        self.code = code


def get_docker_binary() -> str:
    # default to podman
    docker_binary = shutil.which("podman")
    if docker_binary is None:
        docker_binary = shutil.which("docker")
    if docker_binary is None:
        print("Please install docker or podman to run the codegen")
        exit(1)
    return docker_binary


def docker_container_rm(prefix, container_id):
    cmd = [get_docker_binary(), "container", "rm", container_id]
    result = run_cmd(prefix, cmd)
    return result.stdout.decode("utf-8")


def docker_container_logs(prefix, container_id):
    cmd = [get_docker_binary(), "container", "logs", container_id]
    result = run_cmd(prefix, cmd, dont_dbg=True)
    return f"{result.stdout.decode('utf-8')}\n{result.stderr.decode('utf-8')}".strip()


def docker_container_wait(prefix, container_id) -> int:
    cmd = [get_docker_binary(), "container", "wait", container_id]
    result = run_cmd(prefix, cmd)
    return int(result.stdout.decode("utf-8"))


def docker_container_cp(prefix, container_id, task):
    cmd = [
        get_docker_binary(),
        "container",
        "cp",
        f"{container_id}:/app/{task['output_dir']}/.",
        f"{task['output_dir']}/",
    ]
    run_cmd(prefix, cmd)


def docker_container_create(prefix, task) -> str:
    container_name = "codegen-{}-{}-{}".format(
        task["language"],
        task["language_task_index"] + 1,
        "".join(random.choice(string.ascii_lowercase) for _ in range(10)),
    )
    cmd = [
        get_docker_binary(),
        "container",
        "run",
        "-d",
        "--name",
        container_name,
        "--workdir",
        "/app",
        "--mount",
        f"type=bind,src={Path(task['openapi']).absolute()},dst=/app/lib-openapi.json,ro",
        "--mount",
        f"type=bind,src={Path(task['template_dir']).absolute()},dst=/app/{task['template_dir']},ro",
    ]

    for extra_mount_src, extra_mount_dst in task["extra_mounts"].items():
        cmd.append("--mount")
        cmd.append(
            f"type=bind,src={Path(extra_mount_src).absolute()},dst={extra_mount_dst},ro"
        )
    cmd.extend(
        [
            OPENAPI_CODEGEN_IMAGE,
            "openapi-codegen",
            "generate",
            *task["extra_codegen_args"],
            "--template",
            task["template"],
            "--input-file",
            task["openapi"],
            "--output-dir",
            task["output_dir"],
        ]
    )
    run_cmd(prefix, cmd)
    return container_name


def run_cmd(prefix, cmd, dont_dbg=False) -> subprocess.CompletedProcess[bytes]:
    dbg_cmd = [cmd[0], *[f'"{i}"' for i in cmd[1:]]]
    dbg(prefix, f"{BLUE}Running command{ENDC} {' '.join(dbg_cmd)}")
    result = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=REPO_ROOT
    )
    if result.returncode != 0:
        print_cmd_result(result, prefix)
        prefix_print(
            prefix,
            f"subprocess exited with code {result.returncode} (run with DEBUG=yes to see extra logs)",
        )
        raise ExitException(result.returncode)

    if DEBUG and not dont_dbg:
        print_cmd_result(result, prefix)

    return result


def print_cmd_result(result: subprocess.CompletedProcess[bytes], prefix: str):
    for i, stream in enumerate([result.stdout, result.stderr]):
        output = stream.decode("utf-8")
        if output != "":
            cli_prefix = f"{CYAN}{'stdout' if i == 0 else 'stderr'}{ENDC} "
            nice_logs = "{}{}".format(
                cli_prefix,
                output.strip().replace("\n", f"\n{cli_prefix}"),
            )
            prefix_print(prefix, nice_logs)


def dbg(prefix, msg):
    if not DEBUG:
        return
    prefix_print(prefix, msg)


def prefix_print(prefix, msg):
    print(
        "{}{}".format(f"{prefix.strip()} ", msg.replace("\n", f"\n{prefix.strip()} ")),
        flush=True,
    )


def execute_codegen_task(task):
    prefix = "{}{} ({}/{}){} | ".format(
        GREEN,
        task["language"],
        task["language_task_index"] + 1,
        task["language_total"],
        ENDC,
    )

    container_name = docker_container_create(prefix, task).strip()
    dbg(prefix, f"Container id {container_name}")

    exit_code = docker_container_wait(prefix, container_name)

    logs = docker_container_logs(prefix, container_name)

    nice_logs = "{}{}".format(
        f"{CYAN}container logs{ENDC} ",
        logs.strip().replace("\n", f"\n{CYAN}container logs{ENDC} "),
    )

    if exit_code != 0:
        prefix_print(prefix, nice_logs)
        prefix_print(
            prefix,
            f"subprocess exited with code {exit_code} (run with DEBUG=yes to see extra logs)",
        )
        raise ExitException(exit_code)

    dbg(prefix, nice_logs)

    docker_container_cp(prefix, container_name, task)

    docker_container_rm(prefix, container_name)

    print(f"{GREEN}#{ENDC}", flush=True, end="")


def run_codegen_for_language(language, language_config):
    with ThreadPoolExecutor() as pool:
        futures = []
        for t in language_config["tasks"]:
            futures.append(pool.submit(execute_codegen_task, t))
        for future in as_completed(futures):
            if future.exception() is not None:
                raise future.exception()

    extra_shell_commands = language_config.get("extra_shell_commands", [])
    for index, shell_command in enumerate(extra_shell_commands):
        cmd = ["bash", "-c", shell_command]
        run_cmd(
            f"{GREEN}{language}[extra shell commands] ({index + 1}/{len(extra_shell_commands)}){ENDC} | ",
            cmd,
        )


def parse_config():
    with open(REPO_ROOT.joinpath("codegen.toml"), "rb") as f:
        data = tomllib.load(f)
    openapi = data.pop("global")["openapi"]
    config = {}
    for language, language_config in data.items():
        config[language] = {"tasks": [], "tasks_count": len(language_config["task"])}
        for language_task_index, task in enumerate(language_config["task"]):
            config[language]["extra_shell_commands"] = language_config.get(
                "extra_shell_commands", []
            )
            config[language]["tasks"].append(
                {
                    "language": language,
                    "language_task_index": language_task_index,
                    "language_total": len(language_config.get("task", [])),
                    "openapi": task.get(
                        "openapi", language_config.get("openapi", openapi)
                    ),
                    "template": task["template"],
                    "output_dir": task["output_dir"],
                    "extra_mounts": language_config.get("extra_mounts", {}),
                    "extra_codegen_args": task.get("extra_codegen_args", []),
                    "template_dir": language_config["template_dir"],
                }
            )
    # the cli step depends on generated rust code.
    # there is no way to express this dependency in the codegen.toml. so I am hardcoding this here
    cli_config = config.get("cli", None)
    if cli_config is not None and config.get("rust", None) is not None:
        config["rust"]["tasks"].extend(cli_config["tasks"])
        config["rust"]["tasks_count"] += cli_config["tasks_count"]
        config["rust"]["extra_shell_commands"].extend(
            cli_config["extra_shell_commands"]
        )
        config.pop("cli")

    if DEBUG:
        print(json.dumps(config, indent=4), flush=True)
    return config


def pull_image():
    # check if image exists
    check = subprocess.run(
        [
            get_docker_binary(),
            "image",
            "inspect",
            "--format",
            "ignore me",
            OPENAPI_CODEGEN_IMAGE,
        ],
        stderr=subprocess.PIPE,
        stdout=subprocess.PIPE,
    )
    # if it does not exist, pull image
    if check.returncode != 0:
        print("Pulling docker image", flush=True)
        pull = subprocess.run(
            [get_docker_binary(), "image", "pull", OPENAPI_CODEGEN_IMAGE]
        )
        if pull.returncode != 0:
            exit(pull.returncode)


def main():
    pull_image()
    config = parse_config()
    all_tasks = sum([i["tasks_count"] for i in config.values()])
    print(f"Running {all_tasks} codegen tasks")

    # there may be more then 1 subprocess that exited
    exit_with_error = False
    with ThreadPoolExecutor() as pool:
        futures = []
        for language, language_config in config.items():
            futures.append(
                pool.submit(run_codegen_for_language, language, language_config)
            )
        for future in as_completed(futures):
            if future.exception() is not None:
                if isinstance(future.exception(), ExitException):
                    exit_with_error = True
                else:
                    raise future.exception()

    # final newline
    print()
    if exit_with_error:
        exit(1)


if __name__ == "__main__":
    main()
