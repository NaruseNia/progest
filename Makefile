CARGO = cargo
NPM = bun

cli-dev:
	cd ./cli && $(CARGO) run -- $(ARGS)

cli-build:
	cd ./cli && $(CARGO) build --release

app-install:
	cd ./app && $(NPM) install

app-dev: app-install
	cd ./app && $(NPM) tauri dev

app-build: app-install
	cd ./app && $(NPM) tauri build

all: cli-build app-build

clean:
	cd ./cli && $(CARGO) clean
	cd ./app/src-tauri && $(CARGO) clean
