install:
	poetry install
	poetry shell

build: install
	maturin build

dev: install
	maturin develop

test: dev
	poetry run pytest

run: dev
	python3 src/main/python/main.py

docs: dev
	cargo doc --open --document-private-items

lint:
	poetry run flake8

format:
	cargo fmt
	poetry run black src/main/python/
