.PHONY: clean create-env upgrade-env refresh-packages refresh-template reconfigure-template generate-standard-template format lint check validate-tests test test-no-output-file test-one commit push deadcode update-config grant-my-ip-access get-azure-credentials get-entra-token generate-api-templates deploy-connectors
-include *Additional*.mk
-include *Pre*.mk

GREEN=\033[0;32m
BLUE=\033[0;34m
YELLOW=\033[1;33m
NC=\033[0m # No Color

format::
	cargo fmt

build::
	cargo build

lint::
	ruff check .
	ruff format . --check
	MYPYPATH=src/ mypy tools
	MYPYPATH=src/ mypy src
	MYPYPATH=src/ mypy tests

check:: format lint deadcode
	@echo "\n${YELLOW}Formatting and linting complete${NC}\n"

validate-tests::
	PYTHONPATH=./src:. PYTHONDEVMODE=1 pytest --collect-only tests

test::
	mkdir -p logs/tests
	PYTHONPATH=./src:. PYTHONDEVMODE=1 pytest -m "not slow" tests | tee logs/tests/$(shell date +'%Y_%m_%d_%H_%M_%S').log

test-all::
	mkdir -p logs/tests
	PYTHONPATH=./src:. PYTHONDEVMODE=1 pytest tests | tee logs/tests/$(shell date +'%Y_%m_%d_%H_%M_%S').log

test-one::
	mkdir -p logs/tests
	PYTHONPATH=./src:. PYTHONDEVMODE=1 pytest tests -k $(TEST) | tee logs/tests/$(shell date +'%Y_%m_%d_%H_%M_%S').log

test-find-n-slowest::
	PYTHONPATH=./src:. PYTHONDEVMODE=1 pytest --durations=${N} tests

commit::
	@.githooks/pre-commit
	@git add .
	@cz commit

push::
	@(git pull || true) && git push

deadcode::
	deadcode src/

update-config::
	git pull
	PYTHONPATH=. PYTHONDEVMODE=1 python tools/deploy.py --config-only ${CLIENT} ${ENV}

grant-my-ip-access::
	PYTHONPATH=. PYTHONDEVMODE=1 python tools/grant_my_ip_access_to_main_storage_account.py ${CLIENT} ${ENV}

revoke-ip-access::
	PYTHONPATH=. PYTHONDEVMODE=1 python tools/revoke_main_storage_account_access_from_individual_ip_addresses.py ${CLIENT} ${ENV}

render-pyisession-file::
	pyinstrument --load=$(FILE) -o profile.html -r html
	open profile.html
	@echo "\n${YELLOW}Profile output saved to 'profile.html'${NC}\n"

run-fast-api-app-locally::
	PYTHONPATH=src/ uvicorn src.shared_code.services.add_in_service.routes:app --reload

-include react-app/Makefile

get-azure-credentials::
	PYTHONPATH=. PYTHONDEVMODE=1 python tools/get_azure_credentials_for_gh_secret.py ${CLIENT} ${ENV}

get-entra-token::
	PYTHONDEVMODE=1 python tools/get_entra_token.py ${CLIENTID} ${TENANTID} ${SCOPE} --client-secret ${SECRET}


-include *Post*.mk
