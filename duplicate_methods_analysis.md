# Duplicate Service Methods Analysis

## Summary
After analyzing the skillcert_contracts project, I've identified several duplicate service methods that can be consolidated to improve code maintainability and reduce redundancy.

## Identified Duplicates

### 1. Contract Versioning Functions
**Location:** 
- `contracts/course/course_access/src/functions/contract_versioning.rs`
- `contracts/course/course_registry/src/functions/contract_versioning.rs`
- `contracts/user_management/src/functions/contract_versioning.rs`

**Duplicate Code:**
- `VersioningError` enum (identical across all three)
- `get_version_history()` function (identical implementation)
- `store_version_in_history()` function (identical implementation)
- `version_exists_in_history()` function (identical implementation)
- `get_migration_status()` and `set_migration_status()` functions (identical)
- `is_version_compatible()` function (identical)
- `emit_migration_event()` function (identical)
- Storage key constants: `VERSION_HISTORY_KEY`, `MIGRATION_STATUS_KEY`

**Differences:**
- Migration functions are domain-specific:
  - `migrate_access_data()` in course_access
  - `migrate_course_data()` in course_registry  
  - `migrate_user_data()` in user_management
- Authorization checks differ (user_management uses `is_admin()`)

### 2. User Profile Functions
**Location:**
- `contracts/course/course_access/src/functions/save_profile.rs`
- `contracts/user_management/src/functions/save_profile.rs`

**Duplicate Code:**
- Both implement user profile saving functionality
- Similar validation patterns for required fields
- Both use persistent storage for profile data

**Differences:**
- Different function signatures and parameter sets
- Different validation rules (password validation in user_management)
- Different storage schemas and data structures

### 3. Repeated DB Query Patterns
**Location:** Multiple files across contracts

**Duplicate Patterns:**
- Admin configuration retrieval:
  ```rust
  let config: Option<AdminConfig> = env.storage().persistent().get(&DataKey::AdminConfig);
  ```
  Found in: `delete_user.rs`, `rbac.rs`, `is_admin.rs`, `get_user_by_id.rs`

- Course retrieval pattern:
  ```rust
  let course: Course = env.storage().persistent().get(&key).unwrap();
  ```
  Found in multiple course registry functions

- User courses access pattern:
  ```rust
  env.storage().persistent().get::<DataKey, UserCourses>(&user_courses_key)
  ```
  Found in multiple course access functions

## Consolidation Recommendations

### 1. Create Shared Versioning Module
Create a new shared module: `contracts/shared/src/versioning.rs`

**Benefits:**
- Eliminates ~200 lines of duplicate code
- Centralizes versioning logic
- Easier to maintain and update versioning functionality
- Consistent versioning behavior across all contracts

**Implementation:**
- Move common versioning functions to shared module
- Keep domain-specific migration functions in respective contracts
- Use trait-based approach for customizable authorization

### 2. Create Shared Storage Utilities
Create: `contracts/shared/src/storage_utils.rs`

**Benefits:**
- Eliminates repeated DB query patterns
- Provides consistent caching strategies
- Reduces boilerplate code
- Improves performance through standardized caching

**Implementation:**
- Generic functions for common storage operations
- Configurable TTL and caching strategies
- Type-safe storage key management

### 3. Consolidate Profile Management
Create: `contracts/shared/src/profile_utils.rs`

**Benefits:**
- Unified profile validation logic
- Consistent profile data structures
- Reduced code duplication
- Easier to maintain profile-related functionality

**Implementation:**
- Common validation functions
- Shared profile data structures where appropriate
- Configurable validation rules per contract

## Impact Assessment

### Code Reduction
- **Contract Versioning:** ~180 lines of duplicate code across 3 files
- **Storage Patterns:** ~50+ repeated query patterns
- **Profile Functions:** ~100 lines of similar validation logic

### Maintenance Benefits
- Single source of truth for common functionality
- Easier bug fixes and feature updates
- Consistent behavior across contracts
- Reduced testing overhead

### Performance Benefits
- Standardized caching strategies
- Optimized storage access patterns
- Reduced contract size and deployment costs

## Next Steps
1. Create shared module structure
2. Extract common versioning functionality
3. Implement shared storage utilities
4. Update contracts to use shared modules
5. Run comprehensive tests to ensure no functionality is broken
6. Update documentation and deployment scripts
