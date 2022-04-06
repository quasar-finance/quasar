# Parameters 

qoracle module contains below parameters 

# OracleAccounts

OracleAccounts parameter stores the whitelisted bech32 string of the oracle client

Proto message for the OracleAccounts parameter.

// Params defines the parameters for the module.
message Params {
  option (gogoproto.goproto_stringer) = false;
  
  string oracleAccounts = 1 [(gogoproto.moretags) = "yaml:\"oracle_accounts\""];
}