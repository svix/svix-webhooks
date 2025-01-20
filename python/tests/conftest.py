import os
import shutil
from subprocess import CalledProcessError, check_output

import pytest
import requests
from requests.adapters import HTTPAdapter
from svix.api import Svix, SvixOptions
from urllib3.util.retry import Retry

SVIX_ORG_ID = "org_svix_python_tests"
TOKEN = os.getenv("SVIX_TOKEN")
SERVER_URL = os.getenv("SVIX_SERVER_URL")
if not TOKEN and not SERVER_URL:

    def pytest_collection_modifyitems(config, items):
        """Client tests require docker compose (v2 or v1) so skip them if it is not
        installed on host."""
        skipper = None
        if shutil.which("docker") is None:
            skipper = pytest.mark.skip(
                reason="skipping test as docker command is missing"
            )
        else:
            docker_compose_available = False
            try:
                # check if docker compose v2 if available
                check_output(["docker", "compose", "version"])
                docker_compose_available = True
            except CalledProcessError:
                # check if docker compose v1 if available
                docker_compose_available = shutil.which("docker-compose") is not None
            finally:
                if not docker_compose_available:
                    skipper = pytest.mark.skip(
                        reason="skipping test as docker compose is missing"
                    )
        if skipper is not None:
            for item in items:
                if item.module.__name__ == "tests.test_client":
                    item.add_marker(skipper)

    @pytest.fixture(scope="session")
    def docker_compose_command():
        try:
            # use docker compose v2 if available
            check_output(["docker", "compose", "version"])
            return "docker compose"
        except Exception:
            # fallback on v1 otherwise
            return "docker-compose"

    # `pytest-docker` reads this to override the location of the docker-compose.yml file
    @pytest.fixture(scope="session")
    def docker_compose_file():
        return [
            os.path.join(os.path.dirname(__file__), "../../server/docker-compose.yml"),
            os.path.join(os.path.dirname(__file__), "docker-compose.override.yml"),
        ]

    # `pytest-docker` will use this fixture to override the setup command
    @pytest.fixture(scope="session")
    def docker_setup():
        # Don't include the default --build option in the setup
        return ["up -d"]

    @pytest.fixture(scope="session")
    def docker_compose(docker_services):
        return docker_services._docker_compose

    @pytest.fixture(scope="session")
    def httpserver_listen_address():
        # Use IP address in the docker bridge network as server hostname in order for
        # the svix server executed in a docker container to successfully send webhooks
        # to the HTTP server executed on the host
        return ("172.17.0.1", 0)

    @pytest.fixture(scope="session")
    def svix_server_url(docker_services):
        # svix server container exposes a free port to the docker host,
        # we use the docker network gateway IP in case the tests are also
        # executed in a container
        svix_server_port = docker_services.port_for("backend", 8071)
        return f"http://172.17.0.1:{svix_server_port}"

    @pytest.fixture(autouse=True, scope="session")
    def svix_server(svix_server_url):
        """Spawn a Svix server for the tests session using docker compose"""
        # wait for the svix backend service to be up and responding
        request_session = requests.Session()
        retries = Retry(
            total=10, backoff_factor=0.1, status_forcelist=[500, 502, 503, 504]
        )
        request_session.mount("http://", HTTPAdapter(max_retries=retries))
        api_url = f"{svix_server_url}/api/v1/health/"
        response = request_session.get(api_url)
        assert response

    @pytest.fixture(autouse=True)
    def svix_wiper(docker_compose):
        """Ensure stateless tests"""
        yield
        # wipe svix database after each test to ensure stateless tests
        docker_compose.execute(
            f"exec -T backend svix-server wipe --yes-i-know-what-im-doing {SVIX_ORG_ID}"
        )

    @pytest.fixture(scope="session")
    def svix_api(svix_server_url, docker_compose):
        # generate bearer token to authorize communication with the svix server
        exec_output = docker_compose.execute(
            f"exec -T backend svix-server jwt generate {SVIX_ORG_ID}"
        )
        svix_auth_token = (
            exec_output.decode()
            .replace("Token (Bearer): ", "")
            .replace("\r", "")
            .replace("\n", "")
        )
        return Svix(
            svix_auth_token,
            SvixOptions(server_url=svix_server_url),
        )
