# Hooks 

1. Orion module does not have any hooks of its own as of now. 
2. Orion module however registered itself with `x/epoch` module with "day" identifier. And executes its logics in the `AfterEpochEnd` method. 
