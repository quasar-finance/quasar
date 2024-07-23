###############################################################################
###                         Proto & Mock Generation                         ###
###############################################################################
protoVer=0.13.0
protoImageName=ghcr.io/cosmos/proto-builder:$(protoVer)
protoImage=$(DOCKER) run --rm -v $(CURDIR):/workspace --workdir /workspace $(protoImageName)

proto-help:
	@echo "proto subcommands"
	@echo ""
	@echo "Usage:"
	@echo "  make proto-[command]"
	@echo ""
	@echo "Available Commands:"
	@echo "  all        Run proto-format and proto-gen"
	@echo "  gen        Generate Protobuf files"
	@echo "  gen-1      Generate Protobuf files (old relic)"
	@echo "  doc        Generate proto docs"

proto: proto-help
proto-all: proto-format proto-lint proto-gen

# todo : @AJ needs to address this after removing third_party. Refer this for removal https://github.com/osmosis-labs/osmosis/blob/188abfcd15544ca07d468c0dc0169876ffde6079/scripts/makefiles/proto.mk#L39
proto-gen:
	@echo "Generating Protobuf files"
	@$(protoImage) sh ./scripts/protocgen.sh

proto-format:
	@$(protoImage) find ./ -name "*.proto" -exec clang-format -i {} \;

proto-lint:
	@$(protoImage) buf lint --error-format=json

proto-update-deps:
	@echo "Updating Protobuf dependencies"
	$(DOCKER) run --rm -v $(CURDIR)/proto:/workspace --workdir /workspace $(protoImageName) buf mod update

proto-doc:
	@echo "Generating Protoc docs"
	@sh ./scripts/generate-docs.sh