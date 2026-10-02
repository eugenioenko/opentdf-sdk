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

.PHONY: platform-profile-basic platform-profile-ec platform-profile-dpop platform-profile-check interop-profiles
platform-profile-basic:
	./scripts/platform-profile.sh basic
platform-profile-ec:
	./scripts/platform-profile.sh ec
platform-profile-dpop:
	./scripts/platform-profile.sh dpop
platform-profile-check:
	./scripts/platform-profile.sh check
interop-profiles:
	./scripts/platform-profile.sh smoke
