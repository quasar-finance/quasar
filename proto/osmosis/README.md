# Note
This directory contains proto files necessary for IBC communication (Both ICA and ICQ) with osmosis chain.

**Current files were taken from https://github.com/osmosis-labs/osmosis/tree/v12.0.0**

# Upgrade Procedure
To upgrade the proto files to a newer version of osmosis follow the below steps.

## 1. Remove current osmosis proto files
Remove all the files and directories under this directory (`proto/osmosis`) except this file.

## 2. Copy the needed proto files from osmosis
Copy the needed proto files from the latest stable version of osmosis (or any other stable versions desired) with keeping the directory structure of files to this directory.
Note that not all of the module proto files are needed. In most cases for ICA you should copy the `tx.proto` file and for ICQ `query.proto` with all the imported dependency files recursively`.

## 3. Changes in proto files
Change the `go_package` option in all of the imported osmosis proto files from `github.com/osmosis-labs/osmosis/{version}/x/{module}/...` to `github.com/quasarlabs/quasarnode/osmosis/{module}/types`

As an example:
```
option go_package = "github.com/osmosis-labs/osmosis/v12/x/gamm/types";
->
option go_package = "github.com/quasarlabs/quasarnode/osmosis/gamm/types";
```

Remove all the `option (gogoproto.goproto_stringer) = false;` statements from imported osmosis proto files.

## 4. Resolve `sdk.Msg` implementation issues
All the Msg types defined the `tx.proto` files are required to implement the `sdk.Msg` cosmos interface which means we have to implement `GetSigners` and `ValidateBasic` methods for each one of them. But because we only use them in ICA we don't need any special or accurate implementation, just a dummy implementation like below will suffice.
```go
var (
	_ sdk.Msg = &MsgCreateBalancerPool{}
)

func (msg MsgCreateBalancerPool) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgCreateBalancerPool) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}
```

## 5. Generating the go files
To regenerate the go files simply run `make proto-gen` in the root directory of repository.