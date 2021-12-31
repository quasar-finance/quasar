
x/qbank module - 

qbank is the custom bank module for managing user deposit and withdrawal. Internally qbank utlise 
the cosmos-sdk bank module for coins transfer. qbank handles the deposit and withdrawals of users, and also manage the kev-value psersistent store for application level accounting with intelligently used key prefixes for user, vaults, deposit etc. 

Store Key, Getter, Setters Naming - 
{Get/Set}{KeyPrefixName}{ValueTypeName} 

GetUserDenomDepositAmount

