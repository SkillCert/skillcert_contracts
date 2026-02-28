# Off-Chain Data Audit Report

**Project:** SkillCert Contracts  
**Date:** February 24, 2026  
**Purpose:** Identify on-chain data that should be migrated to off-chain storage

---

## Executive Summary

This audit identifies all data structures and fields currently stored on-chain in Soroban smart contracts that are candidates for off-chain storage. Storing extensive string data on-chain is not optimal due to:

- **High storage costs** on blockchain
- **Gas inefficiency** for read/write operations
- **Immutability concerns** for frequently changing data
- **Privacy limitations** for personal information

---

## 1. Data Structures Identified for Off-Chain Migration

### 1.1 UserProfile (contracts/user_profile/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `address` | `Address` | **ESSENTIAL** | Keep on-chain (identity anchor) |
| `name` | `String` | Non-essential | Move off-chain |
| `email` | `Option<String>` | Non-essential | Move off-chain |
| `country` | `String` | Non-essential | Move off-chain |
| `profession` | `String` | Non-essential | Move off-chain |
| `goals` | `String` | Non-essential | Move off-chain |
| `privacy_public` | `bool` | Essential | Keep on-chain (access control) |
| `created_at` | `u64` | Essential | Keep on-chain (audit trail) |
| `updated_at` | `u64` | Essential | Keep on-chain (audit trail) |

---

### 1.2 UserProfile (contracts/course/course_access/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `name` | `String` | Non-essential | Move off-chain |
| `email` | `String` | Non-essential | Move off-chain |
| `profession` | `Option<String>` | Non-essential | Move off-chain |
| `goals` | `Option<String>` | Non-essential | Move off-chain |
| `country` | `String` | Non-essential | Move off-chain |

---

### 1.3 UserProfile (contracts/user_management/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `full_name` | `String` | Non-essential | Move off-chain |
| `contact_email` | `String` | Non-essential | Move off-chain |
| `profession` | `Option<String>` | Non-essential | Move off-chain |
| `country` | `Option<String>` | Non-essential | Move off-chain |
| `purpose` | `Option<String>` | Non-essential | Move off-chain |
| `profile_picture_url` | `Option<String>` | Non-essential | Move off-chain |

---

### 1.4 ProfileUpdateParams (contracts/user_management/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `full_name` | `Option<String>` | Non-essential | Move off-chain |
| `profession` | `Option<String>` | Non-essential | Move off-chain |
| `country` | `Option<String>` | Non-essential | Move off-chain |
| `purpose` | `Option<String>` | Non-essential | Move off-chain |
| `profile_picture_url` | `Option<String>` | Non-essential | Move off-chain |

---

### 1.5 LightProfile (contracts/user_management/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `full_name` | `String` | Non-essential | Move off-chain |
| `profession` | `Option<String>` | Non-essential | Move off-chain |
| `country` | `Option<String>` | Non-essential | Move off-chain |
| `role` | `UserRole` | **ESSENTIAL** | Keep on-chain (access control) |
| `status` | `UserStatus` | **ESSENTIAL** | Keep on-chain (access control) |
| `user_address` | `Address` | **ESSENTIAL** | Keep on-chain (identity anchor) |

---

### 1.6 Course (contracts/course/course_registry/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `id` | `String` | **ESSENTIAL** | Keep on-chain (identifier) |
| `title` | `String` | Non-essential | Move off-chain |
| `description` | `String` | Non-essential | Move off-chain |
| `creator` | `Address` | **ESSENTIAL** | Keep on-chain (ownership) |
| `price` | `u128` | **ESSENTIAL** | Keep on-chain (payment logic) |
| `category` | `Option<String>` | Non-essential | Move off-chain |
| `language` | `Option<String>` | Non-essential | Move off-chain |
| `thumbnail_url` | `Option<String>` | Non-essential | Move off-chain |
| `published` | `bool` | **ESSENTIAL** | Keep on-chain (access control) |
| `prerequisites` | `Vec<CourseId>` | **ESSENTIAL** | Keep on-chain (enrollment logic) |
| `is_archived` | `bool` | **ESSENTIAL** | Keep on-chain (access control) |
| `level` | `Option<CourseLevel>` | Non-essential | Move off-chain |
| `duration_hours` | `Option<u32>` | Non-essential | Move off-chain |

---

### 1.7 CourseModule (contracts/course/course_registry/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `id` | `String` | **ESSENTIAL** | Keep on-chain (identifier) |
| `course_id` | `String` | **ESSENTIAL** | Keep on-chain (relationship) |
| `position` | `u32` | Non-essential | Move off-chain |
| `title` | `String` | Non-essential | Move off-chain |
| `created_at` | `u64` | Essential | Keep on-chain (audit trail) |

---

### 1.8 CourseGoal (contracts/course/course_registry/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `goal_id` | `String` | **ESSENTIAL** | Keep on-chain (identifier) |
| `course_id` | `String` | **ESSENTIAL** | Keep on-chain (relationship) |
| `content` | `String` | Non-essential | Move off-chain |
| `created_by` | `Address` | **ESSENTIAL** | Keep on-chain (ownership) |
| `created_at` | `u64` | Essential | Keep on-chain (audit trail) |

---

### 1.9 CourseCategory (contracts/course/course_registry/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `id` | `u128` | **ESSENTIAL** | Keep on-chain (identifier) |
| `name` | `String` | Non-essential | Move off-chain |
| `description` | `Option<String>` | Non-essential | Move off-chain |

---

### 1.10 EditCourseParams (contracts/course/course_registry/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `new_title` | `Option<String>` | Non-essential | Move off-chain |
| `new_description` | `Option<String>` | Non-essential | Move off-chain |
| `new_price` | `Option<u128>` | **ESSENTIAL** | Keep on-chain |
| `new_category` | `Option<Option<String>>` | Non-essential | Move off-chain |
| `new_language` | `Option<Option<String>>` | Non-essential | Move off-chain |
| `new_thumbnail_url` | `Option<Option<String>>` | Non-essential | Move off-chain |
| `new_published` | `Option<bool>` | **ESSENTIAL** | Keep on-chain |
| `new_level` | `Option<Option<CourseLevel>>` | Non-essential | Move off-chain |
| `new_duration_hours` | `Option<Option<u32>>` | Non-essential | Move off-chain |

---

### 1.11 CourseFilters (contracts/course/course_registry/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `min_price` | `Option<u128>` | **ESSENTIAL** | Keep on-chain |
| `max_price` | `Option<u128>` | **ESSENTIAL** | Keep on-chain |
| `category` | `Option<String>` | Non-essential | Move off-chain |
| `level` | `Option<CourseLevel>` | Non-essential | Move off-chain |
| `min_duration` | `Option<u32>` | Non-essential | Move off-chain |
| `max_duration` | `Option<u32>` | Non-essential | Move off-chain |
| `search_text` | `Option<String>` | Non-essential | Move off-chain |

---

### 1.12 UserFilter (contracts/user_management/src/schema.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `role` | `Option<UserRole>` | **ESSENTIAL** | Keep on-chain |
| `country` | `Option<String>` | Non-essential | Move off-chain |
| `status` | `Option<UserStatus>` | **ESSENTIAL** | Keep on-chain |
| `search_text` | `Option<String>` | Non-essential | Move off-chain |

---

### 1.13 ContractMetadata (contracts/schema_export/src/lib.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `name` | `String` | Non-essential | Move off-chain |
| `version` | `String` | Non-essential | Move off-chain |
| `methods` | `Vec<MethodInfo>` | Non-essential | Move off-chain |

---

### 1.14 MethodInfo (contracts/schema_export/src/lib.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `name` | `String` | Non-essential | Move off-chain |
| `params` | `Vec<ParamInfo>` | Non-essential | Move off-chain |
| `returns` | `String` | Non-essential | Move off-chain |

---

### 1.15 ParamInfo (contracts/schema_export/src/lib.rs)

| Field | Type | On-Chain Necessity | Recommendation |
|-------|------|-------------------|----------------|
| `name` | `String` | Non-essential | Move off-chain |
| `type_name` | `String` | Non-essential | Move off-chain |
| `required` | `bool` | Non-essential | Move off-chain |

---

## 2. Off-Chain Database Schema Mapping

### 2.1 Users Table

```sql
CREATE TABLE users (
    -- Primary key linked to on-chain address
    wallet_address VARCHAR(56) PRIMARY KEY,
    
    -- Profile information (from UserProfile structs)
    full_name VARCHAR(255) NOT NULL,
    contact_email VARCHAR(255) UNIQUE NOT NULL,
    profession VARCHAR(255),
    country VARCHAR(100),
    goals TEXT,
    purpose TEXT,
    profile_picture_url TEXT,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

---

### 2.2 Courses Table

```sql
CREATE TABLE courses (
    -- Primary key linked to on-chain course_id
    course_id VARCHAR(64) PRIMARY KEY,
    
    -- Course metadata (from Course struct)
    title VARCHAR(500) NOT NULL,
    description TEXT,
    category VARCHAR(255),
    language VARCHAR(50),
    thumbnail_url TEXT,
    level VARCHAR(50), -- 'Beginner', 'Intermediate', 'Advanced'
    duration_hours INTEGER,
    
    -- Foreign key to creator (on-chain address)
    creator_address VARCHAR(56) NOT NULL,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

---

### 2.3 Course Modules Table

```sql
CREATE TABLE course_modules (
    -- Primary key linked to on-chain module id
    module_id VARCHAR(64) PRIMARY KEY,
    
    -- Foreign key to course
    course_id VARCHAR(64) NOT NULL REFERENCES courses(course_id),
    
    -- Module metadata (from CourseModule struct)
    position INTEGER NOT NULL,
    title VARCHAR(500) NOT NULL,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

---

### 2.4 Course Goals Table

```sql
CREATE TABLE course_goals (
    -- Primary key linked to on-chain goal_id
    goal_id VARCHAR(64) PRIMARY KEY,
    
    -- Foreign key to course
    course_id VARCHAR(64) NOT NULL REFERENCES courses(course_id),
    
    -- Goal metadata (from CourseGoal struct)
    content TEXT NOT NULL,
    
    -- Foreign key to creator (on-chain address)
    created_by VARCHAR(56) NOT NULL,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

---

### 2.5 Course Categories Table

```sql
CREATE TABLE course_categories (
    -- Primary key linked to on-chain category id
    category_id BIGINT PRIMARY KEY,
    
    -- Category metadata (from CourseCategory struct)
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

---

### 2.6 Contract Metadata Table (Optional)

```sql
CREATE TABLE contract_metadata (
    contract_name VARCHAR(255) PRIMARY KEY,
    version VARCHAR(50) NOT NULL,
    methods JSONB, -- Store method info as JSON
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

---

## 3. Summary of Fields to Move Off-Chain

### 3.1 By Contract File

| File | Struct | Fields to Move Off-Chain |
|------|--------|-------------------------|
| `user_profile/src/schema.rs` | `UserProfile` | `name`, `email`, `country`, `profession`, `goals` |
| `course_access/src/schema.rs` | `UserProfile` | `name`, `email`, `profession`, `goals`, `country` |
| `user_management/src/schema.rs` | `UserProfile` | `full_name`, `contact_email`, `profession`, `country`, `purpose`, `profile_picture_url` |
| `user_management/src/schema.rs` | `ProfileUpdateParams` | `full_name`, `profession`, `country`, `purpose`, `profile_picture_url` |
| `user_management/src/schema.rs` | `LightProfile` | `full_name`, `profession`, `country` |
| `user_management/src/schema.rs` | `UserFilter` | `country`, `search_text` |
| `course_registry/src/schema.rs` | `Course` | `title`, `description`, `category`, `language`, `thumbnail_url`, `level`, `duration_hours` |
| `course_registry/src/schema.rs` | `CourseModule` | `title`, `position` |
| `course_registry/src/schema.rs` | `CourseGoal` | `content` |
| `course_registry/src/schema.rs` | `CourseCategory` | `name`, `description` |
| `course_registry/src/schema.rs` | `EditCourseParams` | `new_title`, `new_description`, `new_category`, `new_language`, `new_thumbnail_url`, `new_level`, `new_duration_hours` |
| `course_registry/src/schema.rs` | `CourseFilters` | `category`, `level`, `min_duration`, `max_duration`, `search_text` |
| `schema_export/src/lib.rs` | `ContractMetadata` | `name`, `version`, `methods` |
| `schema_export/src/lib.rs` | `MethodInfo` | `name`, `params`, `returns` |
| `schema_export/src/lib.rs` | `ParamInfo` | `name`, `type_name`, `required` |
| `user_management/src/models/user.rs` | `UserProfile` | `full_name`, `contact_email`, `profession`, `country`, `purpose`, `profile_picture_url` |

---

### 3.2 Total Count

| Category | Count |
|----------|-------|
| **Structs with off-chain candidates** | 16 |
| **Total string fields to move off-chain** | 47 |
| **Database tables required** | 6 |

---

## 4. Recommended On-Chain Minimal Schema

After migration, on-chain structs should retain only:

### 4.1 Minimal UserProfile (On-Chain)

```rust
#[contracttype]
pub struct UserProfile {
    pub address: Address,
    pub privacy_public: bool,
    pub created_at: u64,
    pub updated_at: u64,
}
```

### 4.2 Minimal Course (On-Chain)

```rust
#[contracttype]
pub struct Course {
    pub id: String,
    pub creator: Address,
    pub price: u128,
    pub published: bool,
    pub prerequisites: Vec<CourseId>,
    pub is_archived: bool,
}
```

### 4.3 Minimal CourseModule (On-Chain)

```rust
#[contracttype]
pub struct CourseModule {
    pub id: String,
    pub course_id: String,
    pub created_at: u64,
}
```

### 4.4 Minimal CourseGoal (On-Chain)

```rust
#[contracttype]
pub struct CourseGoal {
    pub goal_id: String,
    pub course_id: String,
    pub created_by: Address,
    pub created_at: u64,
}
```

### 4.5 Minimal CourseCategory (On-Chain)

```rust
#[contracttype]
pub struct CourseCategory {
    pub id: u128,
}
```

### 4.6 Minimal LightProfile (On-Chain)

```rust
#[contracttype]
pub struct LightProfile {
    pub user_address: Address,
    pub role: UserRole,
    pub status: UserStatus,
}
```

---

## 5. Implementation Notes

1. **Identifier Linking**: On-chain identifiers (`address`, `course_id`, `goal_id`, `module_id`) serve as foreign keys to off-chain database records.

2. **Data Integrity**: Off-chain database should implement constraints to ensure referential integrity with on-chain identifiers.

3. **Indexing Strategy**: Create indexes on `wallet_address`, `course_id`, and other identifier columns for efficient lookups.

4. **Sync Mechanism**: Implement event listeners or polling to keep off-chain data synchronized with on-chain state changes.

5. **Privacy Considerations**: The `privacy_public` flag should be checked before exposing off-chain profile data via APIs.

---

## 6. Files Analyzed

| File Path | Structs Analyzed |
|-----------|-----------------|
| `contracts/user_profile/src/schema.rs` | `UserProfile`, `DataKey` |
| `contracts/course/course_access/src/schema.rs` | `CourseAccess`, `UserCourses`, `DataKey`, `UserProfile`, `CourseUsers` |
| `contracts/course/course_registry/src/schema.rs` | `CourseModule`, `CourseGoal`, `CourseRateLimitConfig`, `CourseRateLimitData`, `CourseCategory`, `DataKey`, `Course`, `CourseId`, `Category`, `CourseFilters`, `EditCourseParams`, `CourseBackupData` |
| `contracts/user_management/src/schema.rs` | `UserProfile`, `ProfileUpdateParams`, `UserRole`, `Permission`, `RolePermissions`, `UserPermissions`, `UserStatus`, `UserFilter`, `LightProfile`, `RateLimitConfig`, `RateLimitData`, `AdminConfig`, `UserBackupData`, `PaginationParams`, `PaginatedLightProfiles`, `DataKey` |
| `contracts/user_management/src/models/user.rs` | `UserProfile` |
| `contracts/schema_export/src/lib.rs` | `ContractMetadata`, `MethodInfo`, `ParamInfo` |

---

*End of Audit Report*
