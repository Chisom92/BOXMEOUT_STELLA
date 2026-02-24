# Issue Resolution: Implement deregister_oracle in Oracle Contract

## Implementation Summary

### Function Added: `deregister_oracle()`

**Location:** `contracts/contracts/boxmeout/src/oracle.rs`

**Signature:**
```rust
pub fn deregister_oracle(env: Env, oracle: Address)
```

**Features:**
✅ Admin-only function (requires admin authentication)
✅ Removes oracle from active set (marks as inactive)
✅ Recalculates consensus threshold automatically
✅ Emits OracleDeregistered event
✅ Comprehensive unit tests

**Implementation Details:**

1. **Admin Authentication**
   - Requires admin signature via `admin.require_auth()`
   - Retrieves admin from persistent storage

2. **Validation**
   - Checks if oracle is currently registered
   - Panics with "Oracle not registered" if not found

3. **Remove from Active Set**
   - Sets oracle status to `false` (marks as inactive)
   - Preserves oracle data for historical records
   - Prevents oracle from submitting new attestations

4. **Update Oracle Count**
   - Decrements `ORACLE_COUNT_KEY` by 1
   - Safely handles edge case where count is already 0

5. **Recalculate Consensus Threshold**
   - Adjusts threshold if remaining oracles < current threshold
   - Ensures threshold never exceeds available oracles
   - Formula: `new_threshold = min(oracle_count - 1, required_consensus)`

6. **Emit Event**
   - Publishes `OracleDeregistered` event
   - Includes oracle address and timestamp

## Unit Tests Added

**Location:** `contracts/contracts/boxmeout/src/oracle.rs` (tests module)

### Test Cases:

1. **test_deregister_oracle**
   - Verifies oracle is marked as inactive after deregistration
   - Confirms oracle status changes from `true` to `false`

2. **test_deregister_unregistered_oracle**
   - Tests error handling for unregistered oracle
   - Expects panic with "Oracle not registered"

3. **test_deregister_oracle_recalculates_threshold**
   - Validates consensus threshold adjustment
   - Confirms threshold reduces from 2 to 1 when oracle count drops

4. **test_deregister_oracle_decrements_count**
   - Verifies oracle count decreases correctly
   - Confirms count changes from 2 to 1 after deregistration

## Acceptance Criteria Met

✅ **Admin-only function** - Requires admin authentication
✅ **Remove oracle from active set** - Sets oracle status to false
✅ **Recalculate consensus threshold** - Adjusts threshold based on remaining oracles
✅ **Emit OracleDeregistered event** - Event published with oracle address and timestamp
✅ **Unit tests** - 4 comprehensive test cases covering all scenarios

## Usage Example

```rust
// Admin deregisters a misbehaving oracle
oracle_client.deregister_oracle(&oracle_address);

// Oracle is now inactive and cannot submit attestations
// Existing attestations remain valid for historical records
// Consensus threshold automatically adjusted if needed
```

## Integration Notes

- Deregistered oracles cannot submit new attestations (checked in `submit_attestation`)
- Existing attestations from deregistered oracles remain valid
- Consensus threshold automatically adjusts to prevent deadlock
- Oracle data preserved for audit trail and historical analysis
- Can be called multiple times safely (idempotent after first call)

## Edge Cases Handled

1. **Unregistered Oracle** - Panics with clear error message
2. **Zero Oracle Count** - Safely handles without underflow
3. **Threshold Adjustment** - Ensures threshold ≤ remaining oracles
4. **Already Deregistered** - Panics on second attempt (oracle not registered)
