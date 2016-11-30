CI_BUILD_NUMBER ?= $(USER)-snapshot

VERSION ?= 0.1.$(CI_BUILD_NUMBER)

BUILDER_TAG=ekidd/rust-musl-builder:1.13.0
BUILDER_DIR=/home/rust/src
#BUILDER_TAG=clux/muslrust:1.15.0-nightly-2016-11-29
#BUILDER_DIR=/volume
PUBLISH_TAG=meetup/jira-transit:$(VERSION)

# For faster local builds, try a caching container
# Keep a stopped container for volumes:
#   docker run -v /home/rust/.cargo --name cargo-cache $(BUILDER_TAG) echo "running"
# then
#   export CI_CARGO_CACHE="--volumes-from cargo-cache"
CI_CARGO_CACHE ?=

# lists all available targets
list:
	@sh -c "$(MAKE) -p no_op__ | \
		awk -F':' '/^[a-zA-Z0-9][^\$$#\/\\t=]*:([^=]|$$)/ {split(\$$1,A,/ /);\
		for(i in A)print A[i]}' | \
		grep -v '__\$$' | \
		grep -v 'make\[1\]' | \
		grep -v 'Makefile' | \
		sort"

# required for list
no_op__:

# Assemles the software artifact using the defined build image.
__package:
	docker pull $(BUILDER_TAG)
	# build static binary
	docker run \
	  --rm -it \
		-v "$(PWD)":$(BUILDER_DIR) \
		-w $(BUILDER_DIR) \
		$(CI_CARGO_CACHE) \
		$(BUILDER_TAG) \
		cargo build --release
	# build service container
	docker build -t $(PUBLISH_TAG)

component-test:
	@echo "Not implemented yet"

package: __package test component-test

#Pushes the container to the docker registry/repository.
publish: package
	@docker push $(PUBLISH_TAG)

version:
	@echo $(VERSION)

publish-tag:
	@echo $(PUBLISH_TAG)

validate:
# Kubectl doesn't allow dry run validation on everything just yet
# but when it does this could be useful here.
	@echo "Not implemented yet"

test:
	docker pull $(BUILDER_TAG)
	docker run \
		--rm -it \
		-v "$(PWD)":$(BUILDER_DIR) \
		-w $(BUILDER_DIR) \
		$(CI_CARGO_CACHE) \
		$(BUILDER_TAG) \
		cargo test
