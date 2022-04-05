install:
	poetry install

build:
	maturin develop

run:
	python3 src/main/python/main.py
