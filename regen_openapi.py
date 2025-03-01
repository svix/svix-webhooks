#!/usr/bin/env python3
import pathlib
from pathlib import Path
import shutil
import subprocess
import os
from concurrent.futures import ThreadPoolExecutor

try:
    import tomllib
except ImportError:
    print("Python 3.11 or greater is required to run the codegen")
    exit(1)

OPENAPI_CODEGEN_IMAGE = "ghcr.io/svix/openapi-codegen:latest"
REPO_ROOT = pathlib.Path(__file__).parent.resolve()
DEBUG = os.getenv("DEBUG") is not None


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
    result = run_cmd(prefix, cmd)
    return result.stdout.decode("utf-8")


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
    cmd = [
        get_docker_binary(),
        "container",
        "run",
        "-d",
        "--workdir",
        "/app",
        "--mount",
        f"type=bind,src={Path(task['openapi']).absolute()},dst=/app/lib-openapi.json,ro",
        "--mount",
        f"type=bind,src={Path(task['template_dir']).absolute()},dst=/app/{task['template_dir']},ro",
    ]
    for extra_mount in task["extra_mounts"]:
        cmd.append("--mount")
        cmd.append(
            f"type=bind,src={Path(extra_mount[0]).absolute()},dst={extra_mount[1]},ro"
        )
    cmd.extend(
        [
            OPENAPI_CODEGEN_IMAGE,
            "openapi-codegen",
            "generate",
            "--create-file-parents",
            "--template",
            task["template"],
            "--input-file",
            task["openapi"],
            "--output-dir",
            task["output_dir"],
        ]
    )
    result = run_cmd(prefix, cmd)
    return result.stdout.decode("utf-8")


def run_cmd(prefix, cmd) -> subprocess.CompletedProcess[bytes]:
    dbg(prefix, " ".join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE, cwd=REPO_ROOT)
    if result.returncode != 0:
        print_cmd_result(result, prefix)
        exit(result.returncode)
    return result


def print_cmd_result(result: subprocess.CompletedProcess[bytes], prefix: str):
    for i in [result.stdout, result.stderr]:
        if i is None:
            continue
        prefix_print(prefix, i.decode("utf-8"))


def dbg(prefix, msg):
    if not DEBUG:
        return
    prefix_print(prefix, msg)


def prefix_print(prefix, msg):
    prefix = f"{prefix.strip()} | "
    print("{}{}".format(prefix, msg.strip().replace("\n", f"\n{prefix} ")))


def execute_codegen_task(task):
    prefix = f"{task['language']}[{task['language_index']+1}/{task['language_total']}]"
    prefix_print(prefix, "Starting codegen task")

    container_id = docker_container_create(prefix, task).strip()
    dbg(prefix, f"Container id {container_id}")

    exit_code = docker_container_wait(prefix, container_id)

    logs = docker_container_logs(prefix, container_id)
    prefix_print(prefix, logs)

    if exit_code != 0:
        exit(exit_code)

    docker_container_cp(prefix, container_id, task)

    docker_container_rm(prefix, container_id)


def run_codegen_for_language(language, language_config):
    tasks = []
    for index, task in enumerate(language_config.get("task", [])):
        tasks.append(
            {
                "language": language,
                "language_index": index,
                "language_total": len(language_config.get("task", [])),
                "openapi": language_config["openapi"],
                "template": task["template"],
                "output_dir": task["output_dir"],
                "extra_mounts": language_config.get("extra_mounts", []),
                "template_dir": language_config["template_dir"],
            }
        )
    with ThreadPoolExecutor() as pool:
        pool.map(execute_codegen_task, tasks)
    pool.shutdown(wait=True)

    for shell_command in language_config.get("extra_shell_commands"):
        cmd = ["bash", "-c", shell_command]
        run_cmd(language, cmd)


def main():
    # TODO: pull image before running tasks
    with ThreadPoolExecutor() as pool:
        with open(REPO_ROOT.joinpath("codegen.toml"), "rb") as f:
            data = tomllib.load(f)
        for language, language_config in data.items():
            pool.submit(run_codegen_for_language, language, language_config)

    pool.shutdown(wait=True)


if __name__ == "__main__":
    main()
