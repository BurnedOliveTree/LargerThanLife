# GameOfLife

## Setting up the environment
------------

1. Install `poetry`: https://python-poetry.org/docs/#installation
2. Create an environment with `poetry install`
3. Run `poetry shell`
4. Run `maturin develop` to attach the rust binaries as a Python module to the venv
5. To add a new package run `poetry add <package>`. Don't forget to commit the lockfile.
6. To run unit tests for your service use `poetry run pytest` or simply `pytest` within `poetry shell`.

## Commands
### Run test
> poetry run pytest
### Run black
> poetry run black ltl_game/
### Run flake8
> poetry run flake8
### Build rust library
> maturin build