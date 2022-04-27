MODULES:=$(CURDIR)/modules
PLATFORM:=$(CURDIR)/platform

clean:
	rm $(PLATFORM)/*.wasm
	rm -rf $(MODULES)/hello-world-as/build

compile-faas:
	cd $(PLATFORM) \
	&& cargo build --release

compile-wasm: compile-hello-world

copy-wasm: compile-wasm copy-wasm

compile-hello-world:
	cd $(MODULES)/hello-world-as \
	&& npx asc assembly/index.ts --target release -o build/hello-world.wasm \
	&& cp build/hello-world.wasm $(PLATFORM)/hello-world.wasm

copy-hello-world: compile-hello-world
	cd $(MODULES)/hello-world-as \
	&& cp build/hello-world.wasm $(PLATFORM)/hello-world.wasm

serve: compile-faas compile-wasm
	cd $(PLATFORM) && cargo run --release