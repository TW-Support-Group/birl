REMOTE_USER := aloussase
REMOTE_IP := jupiter
PROJECT := birl
REMOTE_PATH := /home/$(REMOTE_USER)

build:
	@echo "Building executable for platform aarch64"
	@cross build --target aarch64-unknown-linux-gnu --release >/dev/null 2> build.log
	@echo "Copying executable to remote host"
	@scp target/aarch64-unknown-linux-gnu/release/$(PROJECT) $(REMOTE_USER)@$(REMOTE_IP):$(REMOTE_PATH)/birl-bot
	@echo "Copying systemd service file to remote host"
	@scp birl.service $(REMOTE_USER)@$(REMOTE_IP):$(REMOTE_PATH)/birl.service
