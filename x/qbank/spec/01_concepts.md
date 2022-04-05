# Concepts

The module `qbank` facilitates the end users deposit, withdraw and claim functionalities for the various vaults in the quasar chain.

qbanks maintains the data structure storage for users deposit done on a specfic epoch day for an input denom and for how many days is lockuped since the deposit epoch day.

Internally users won't directly communicates with the vaults. The deposit, withdraw and claim operations done by the users from the front end on any vault are actually done though message transactions on the qbank module. Except if there is any specific vault that wants the message processing at the same time when it is delivered to the chain.

When a message transaction is broadcasted from the frontend application on the qbank module it will store process the message and store the data in appropriate prefixed KV store so it can be later used as needed by the vaults.

qbank maintains the details of users total deposit, users deposit for a specific coin denom, users deposits on any specific epochday, and users deposit lockup informations. This makes it very useful for the vaults to fetch deposit funds and execute their strategy logic. Based on the lockup days information stored by the qbank, vaults knows in advanced that how many days a particular funds is available for yield farming.
