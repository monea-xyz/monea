ethereum_package = import_module("github.com/ethpandaops/ethereum-package/main.star")
contract_deployer = import_module("./src/contracts/contract_deployer.star")
static_files = import_module(
    "github.com/ethpandaops/ethereum-package/src/static_files/static_files.star"
)
l2_launcher = import_module("./src/l2.star")
wait_for_sync = import_module("./src/wait/wait_for_sync.star")
input_parser = import_module("./src/package_io/input_parser.star")


def run(plan, args):
    plan.print("Parsing the L1 input args")
    
    # If no args are provided, use the default values with minimal preset
    # ethereum_args = args.get("ethereum_package", input_parser.default_ethereum_config())

    # Deploy the L1
    plan.print("Deploying a local L1")
    l1 = ethereum_package.run(plan, args)

    l1_config_string = "".join(["'", json.encode(l1), "'"])

    plan.print(l1_config_string)

    # write the returned l1 data to a file
    plan.run_sh(
        name = "l1_config_artifact_writer",
        image = "file-writer",
        store = [
          StoreSpec(
            name = "l1_config",
            src = "/l1_config.json"
          )
        ],
        run = " ".join(["file_writer", l1_config_string, "/l1_config.json"])
    )

    return l1
