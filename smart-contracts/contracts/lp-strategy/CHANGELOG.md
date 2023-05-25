# CHANGELOG

## Unreleased
### Dependencies
### API breaking
### State breaking
- Remove old pending acks from the state
### Improvements
- Add bond queue duplicate key check
- Add testing to try_icq
- Remove unnecessary load from try_icq
- Changed the locking on the execute calls to lock correctly depending on queue state
- Remove pending ack entry after succesful ack handling
### Features
### Bugfixes
- Compare users' shares to their owned amount of queued shares instead of all queued shares 

## V0.1.1 08-05-2023
### Dependencies
### API breaking
### State breaking
- Recover user bonds with manual callbacks
### Improvements
### Features
### Bugfixes
### Notes

Migrations (branch names) performed with this source:
migration-004/recover-bonds
migration-005/recover-bonds-again

## V0.1.0
### Initial version