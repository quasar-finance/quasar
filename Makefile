go.mod:
	go mod tidy && go mod verify

test_path:
	go test -v $(path)

test:
	@$(MAKE) test_path path="./x/epochs/... ./x/intergamm/... ./x/qbank/..."

test_simulation:
	ignite chain simulate -v

docs_gen:
	scripts/gen_grpc_doc

docs_serve:
	scripts/serve_doc_docker

run:
	./run.sh

run_silent:
	./run.sh > q.log 2>&1

.PHONY: go.mod test_path test test_simulation docs_gen docs_serve run run_silent
