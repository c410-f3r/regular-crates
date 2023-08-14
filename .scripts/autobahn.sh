set -euxo pipefail

ARG=${1:-""}
if [ "$ARG" != "ci" ]; then
	trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
fi;

# fuzzingclient

cargo build --release --example web-socket-hyper-echo-server --features hyper
cargo run --release --example web-socket-hyper-echo-server --features hyper &
mkdir -p .scripts/autobahn/reports/fuzzingclient
podman rm --force --ignore fuzzingclient
podman run \
	-v .scripts/autobahn/fuzzingclient.json:/fuzzingclient.json:ro \
	-v .scripts/autobahn:/autobahn \
	--name fuzzingclient \
	--net=host \
	--rm \
	docker.io/crossbario/autobahn-testsuite:0.8.2 wstest -m fuzzingclient -s fuzzingclient.json
podman rm --force --ignore fuzzingclient

if [ $(grep -ci "failed" .scripts/autobahn/reports/fuzzingclient/index.json) -gt 0 ]
then
    exit 1
fi

## fuzzingserver

mkdir -p .scripts/autobahn/reports/fuzzingserver
podman rm --force --ignore fuzzingserver
podman run \
	-d \
	-p 8081:8081 \
	-v .scripts/autobahn/fuzzingserver.json:/fuzzingserver.json:ro \
	-v .scripts/autobahn:/autobahn \
	--name fuzzingserver \
	--net=host \
	docker.io/crossbario/autobahn-testsuite:0.8.2 wstest -m fuzzingserver -s fuzzingserver.json
sleep 1
cargo run --release --example web-socket-hyper-autobahn-client --features hyper
podman rm --force --ignore fuzzingserver

if [ $(grep -ci "failed" .scripts/autobahn/reports/fuzzingserver/index.json) -gt 0 ]
then
    exit 1
fi
