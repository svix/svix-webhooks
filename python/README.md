# Svix

The Svix client API

## Usage

```sh
pip install Svix
```

```python
from svix.api import Svix, ApplicationIn

svix = Svix("AUTH_TOKEN")
app = svix.application.create(ApplicationIn(name="Application name"))
```

Please refer to [the documentation](https://docs.svix.com/) or [the API reference](https://api.svix.com/docs) for usage instructions.


## Development

### Install Deps

```sh
python -m venv .venv
pip install -r requirements.txt && pip install -r requirements-dev.txt
```

### Run Tests

```sh
pytest
```
