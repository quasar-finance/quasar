###############################################################################
###                         Proto & Mock Generation                         ###
###############################################################################
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
proto-all: proto-gen

# todo : @AJ needs to address this after removing third_party. Refer this for removal https://github.com/osmosis-labs/osmosis/blob/188abfcd15544ca07d468c0dc0169876ffde6079/scripts/makefiles/proto.mk#L39
proto-gen:
	@echo "Generating Protobuf files"
	@sh ./scripts/protocgen.sh

# todo : @AK need the reason why it was there earlier
proto-gen-1:
	@echo "🤖 Generating code from protobuf..."
	@echo "PWD is $(PWD)"

	@docker run --rm --volume "$(PWD)":/workspace --workdir /workspace \
		ghcr.io/cosmos/proto-builder:$(BUILDER_VERSION) sh ./scripts/protocgen.sh
	@echo "✅ Completed code generation!"

proto-doc:
	@echo "Generating Protoc docs"
	@sh ./scripts/generate-docs.sh