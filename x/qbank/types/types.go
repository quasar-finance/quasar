package types

var (
	// Note - New vault to be added everytime a new vault is created and to be supported by qbank
	SupportedVaultTypes = []string{"orion"}
)

func Contains(inputslice []string, inputElement string) bool {
	for _, v := range inputslice {
		if v == inputElement {
			return true
		}
	}
	return false
}
