.PHONY: platform-init platform-build platform-up platform-down platform-ready platform-status platform-logs interop-smoke
platform-init:
	./scripts/platform.sh init
platform-build:
	./scripts/platform.sh build
platform-up:
	./scripts/platform.sh up
platform-down:
	./scripts/platform.sh down
platform-ready:
	./scripts/platform.sh ready
platform-status:
	./scripts/platform.sh status
platform-logs:
	./scripts/platform.sh logs
interop-smoke:
	./scripts/platform.sh smoke
