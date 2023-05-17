# CHANGELOG

## Unreleased
### Dependencies
### API breaking
### State breaking
- Remove old pending acks from the state
### Improvements
- Remove pending ack entry after succesful ack handling
- Add testing to try_icq
- Remove unnecessary load from try_icq
- Changed the locking on the execute calls to lock correctly depending on queue state
### Features
- Change load() for should_load() to get namespaces & keys on errors
### Bugfixes

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