use axos_primitives::SyncMode;

fn main() {
    let network = "optimism";
    let l1_rpc_url = "https://eth-mainnet.g.alchemy.com/v2/<API_KEY>";
    let l1_ws_rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/<API_KEY>";
    let jwt_secret = "bf549f5188556ce0951048ef467ec93067bc4ea21acebe46ef675cd4e8e015ff";
    let rpc_port = 9545;
    let execution_client = "op-geth";
    let execution_client_auth_rpc_port = 8551;
    let execution_client_rpc_port = 8545;
    let execution_client_ws_port = 8546;
    let sync_mode = SyncMode::Full;
    let l1_test_rpc_url = "https://eth-goerli.g.alchemy.com/v2/<API_KEY>";
    let l2_test_rpc_url = "https://opt-goerli.g.alchemy.com/v2/<API_KEY>";
}
