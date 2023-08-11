# CHANGELOG

## Unreleased

### Dependencies

### API breaking

### State breaking

- Delete old pending acks from the state
- Delete failed traps from the state

### Improvements

- Add bond queue duplicate key check
- Add testing to try_icq
- Remove unnecessary load from try_icq
- Changed the locking on the execute calls to lock correctly depending on queue state
- Delete pending ack entry after succesful ack handling
- Added some doc comments
- Created execute.rs file and created retry exit pool fn there
- Added proptests for retry join pool

### Features

- Added retry entry point to handle exit pool errors
- Added retry entry point to handle join pool errors

### Bugfixes

- divide quote denom by spotprice instead of multiply with spotprice
- readd proper lock behaviour
- Do not allow opentry messages to clog up our state
- Compare users' shares to their owned amount of queued shares instead of all queued shares
- make it so that the primitive compounds
- using only the unbonds amount to calculate slippage (previously using total shares amount)
- fixed math on consolidate_exit_pool_amount_into_local_denom

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
