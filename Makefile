CARGO = cargo
NPM = pnpm

cli-dev:
	cd ./cli && $(CARGO) run

cli-build:
	cd ./cli && $(CARGO) build --release

app-install:
	cd ./app && $(NPM) install

app-dev: app-install
	cd ./app && $(CARGO) tauri dev

app-build: app-install
	cd ./app && $(CARGO) tauri build

all: cli-build app-build

clean:
	cd ./cli && $(CARGO) clean
	cd ./app/src-tauri && $(CARGO) clean
