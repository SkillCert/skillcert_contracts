// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

//! Shared utilities and common functionality for SkillCert contracts
//! 
//! This module provides reusable components that are used across multiple
//! contracts to reduce code duplication and ensure consistency.

pub mod versioning;
pub mod storage_utils;
pub mod profile_utils;

// Re-export commonly used types and functions
pub use versioning::*;
pub use storage_utils::*;
pub use profile_utils::*;
