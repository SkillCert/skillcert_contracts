import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}




export const Errors = {
  1: {message:"AlreadInitialized"},
  2: {message:"InvalidMaxPageSize"},
  3: {message:"SystemNotInitialized"},
  4: {message:"AccessDenied"},
  5: {message:"SuperAdminNotRegular"},
  6: {message:"OperationFailed"},
  7: {message:"MaxAdminsReached"},
  8: {message:"CannotRemoveSuperAdmin"},
  9: {message:"UserProfileExists"},
  10: {message:"NameRequired"},
  11: {message:"EmailRequired"},
  12: {message:"CountryRequired"},
  15: {message:"InvalidEmailFormat"},
  16: {message:"EmailAlreadyExists"},
  17: {message:"InvalidField"},
  19: {message:"InvalidProfilePicURL"},
  20: {message:"UserNotFound"},
  21: {message:"UserProfileNotFound"},
  22: {message:"InactiveUser"},
  23: {message:"PageParamTooLarge"},
  24: {message:"InvalidTitleLength"},
  25: {message:"PasswordMismatch"},
  26: {message:"RateLimitExceeded"},
  27: {message:"RateLimitNotConfigured"},
  28: {message:"PasswordTooShort"},
  29: {message:"PasswordTooLong"},
  30: {message:"PasswordMissingUppercase"},
  31: {message:"PasswordMissingLowercase"},
  32: {message:"PasswordMissingDigit"},
  33: {message:"PasswordMissingSpecialChar"},
  34: {message:"RequiredFieldMissing"},
  35: {message:"Unauthorized"}
}

/**
 * Errors that can occur during contract versioning operations
 */
export const VersioningError = {
  /**
   * Invalid version format
   */
  1: {message:"InvalidVersion"},
  /**
   * Version not found in history
   */
  2: {message:"VersionNotFound"},
  /**
   * Migration not compatible
   */
  3: {message:"MigrationNotCompatible"},
  /**
   * Migration already completed
   */
  4: {message:"MigrationAlreadyCompleted"},
  /**
   * Unauthorized migration attempt
   */
  5: {message:"UnauthorizedMigration"},
  /**
   * Migration failed
   */
  6: {message:"MigrationFailed"}
}


export interface UserProfile {
  /**
 * User's contact email address (required, must be unique)
 */
contact_email: string;
  /**
 * User's country of residence (optional)
 */
country: Option<string>;
  /**
 * User's full name (required)
 */
full_name: string;
  /**
 * User's profession or job title (optional)
 */
profession: Option<string>;
  /**
 * User's profile picture URL (optional)
 */
profile_picture_url: Option<string>;
  /**
 * User's learning goals or purpose (optional)
 */
purpose: Option<string>;
}

/**
 * Data keys for contract storage
 * 
 * Currently includes only UserProfile keyed by user Address
 */
export type DataKey = {tag: "UserProfile", values: readonly [string]} | {tag: "EmailIndex", values: readonly [string]};


/**
 * User profile information matching UI definition.
 * 
 * This struct contains user profile data with required and optional fields
 * as defined by the user interface requirements.
 */
export interface UserProfile {
  /**
 * User's contact email address (required, must be unique)
 */
contact_email: string;
  /**
 * User's country of residence (optional)
 */
country: Option<string>;
  /**
 * User's full name (required)
 */
full_name: string;
  /**
 * User's profession or job title (optional)
 */
profession: Option<string>;
  /**
 * User's profile picture URL (optional)
 */
profile_picture_url: Option<string>;
  /**
 * User's learning goals or purpose (optional)
 */
purpose: Option<string>;
}


/**
 * Struct for profile update parameters
 * Only includes fields that can be updated
 */
export interface ProfileUpdateParams {
  /**
 * User's country of residence
 */
country: Option<string>;
  /**
 * User's full name (optional update)
 */
full_name: Option<string>;
  /**
 * User's profession or job title
 */
profession: Option<string>;
  /**
 * User's profile picture URL
 */
profile_picture_url: Option<string>;
  /**
 * User's learning goals or purpose
 */
purpose: Option<string>;
}

/**
 * User roles in the SkillCert platform.
 * 
 * Defines the different types of users and their permission levels.
 */
export type UserRole = {tag: "Student", values: void} | {tag: "Instructor", values: void} | {tag: "Admin", values: void} | {tag: "SuperAdmin", values: void} | {tag: "Moderator", values: void} | {tag: "Support", values: void};

/**
 * Granular permissions for RBAC system.
 * 
 * Defines specific actions that can be granted or denied to users.
 */
export type Permission = {tag: "ViewUsers", values: void} | {tag: "EditUsers", values: void} | {tag: "DeleteUsers", values: void} | {tag: "CreateUsers", values: void} | {tag: "ViewCourses", values: void} | {tag: "CreateCourses", values: void} | {tag: "EditCourses", values: void} | {tag: "DeleteCourses", values: void} | {tag: "ManageCourseAccess", values: void} | {tag: "ManageSystem", values: void} | {tag: "ManageAdmins", values: void} | {tag: "ViewAnalytics", values: void} | {tag: "ModerateContent", values: void} | {tag: "ProvideSupport", values: void} | {tag: "ViewSupport", values: void};


/**
 * Role-based permissions mapping.
 * 
 * Defines which permissions are granted to each role by default.
 */
export interface RolePermissions {
  /**
 * List of permissions granted to this role
 */
permissions: Array<Permission>;
  /**
 * The role this permission set applies to
 */
role: UserRole;
}


/**
 * User-specific permission overrides.
 * 
 * Allows granting or revoking specific permissions to individual users.
 */
export interface UserPermissions {
  /**
 * Additional permissions granted beyond role defaults
 */
granted_permissions: Array<Permission>;
  /**
 * Permissions explicitly revoked from role defaults
 */
revoked_permissions: Array<Permission>;
  /**
 * The user address
 */
user: string;
}

/**
 * User account status.
 * 
 * Represents the current state of a user's account.
 */
export type UserStatus = {tag: "Active", values: void} | {tag: "Inactive", values: void} | {tag: "Suspended", values: void};


/**
 * Lightweight user profile for listing operations.
 * 
 * Contains essential user information for efficient querying and display in user lists.
 */
export interface LightProfile {
  /**
 * User's country of residence
 */
country: Option<string>;
  /**
 * User's full name
 */
full_name: string;
  /**
 * User's profession or job title
 */
profession: Option<string>;
  /**
 * User's role in the platform
 */
role: UserRole;
  /**
 * User's account status
 */
status: UserStatus;
  /**
 * User's blockchain address
 */
user_address: string;
}


/**
 * Rate limiting configuration for user operations.
 * 
 * Tracks rate limiting settings and current usage for spam protection.
 */
export interface RateLimitConfig {
  /**
 * Maximum operations allowed per window
 */
max_operations_per_window: u32;
  /**
 * Time window for rate limiting in seconds
 */
window_seconds: u64;
}


/**
 * Rate limiting tracking data for a specific address.
 * 
 * Stores the current usage count and window start time for rate limiting.
 */
export interface RateLimitData {
  /**
 * Current count of operations in this window
 */
count: u32;
  /**
 * Timestamp when the current window started
 */
window_start: u64;
}


/**
 * Administrative configuration for the user management system.
 * 
 * Contains system-wide settings and administrative information.
 */
export interface AdminConfig {
  /**
 * Whether the system has been initialized
 */
initialized: boolean;
  /**
 * Maximum allowed page size for queries
 */
max_page_size: u32;
  /**
 * Rate limiting configuration for user creation
 */
rate_limit_config: RateLimitConfig;
  /**
 * Address of the super administrator
 */
super_admin: string;
  /**
 * Total number of registered users
 */
total_user_count: u32;
}


/**
 * Backup data structure for user management system.
 * 
 * Contains all user data and system configuration for backup and recovery operations.
 */
export interface UserBackupData {
  /**
 * Administrative configuration
 */
admin_config: AdminConfig;
  /**
 * List of admin addresses
 */
admins: Array<string>;
  /**
 * Backup timestamp
 */
backup_timestamp: u64;
  /**
 * Backup version for compatibility
 */
backup_version: string;
  /**
 * Email to address mapping for uniqueness
 */
email_mappings: Map<string, string>;
  /**
 * All lightweight profiles for efficient queries
 */
light_profiles: Map<string, LightProfile>;
  /**
 * All user profiles in the system
 */
user_profiles: Map<string, UserProfile>;
  /**
 * List of all registered user addresses
 */
users_index: Array<string>;
}


/**
 * Pagination parameters for cursor-based pagination.
 * 
 * Used to implement efficient pagination that avoids gas limit issues
 * with large datasets by using cursor-based navigation.
 */
export interface PaginationParams {
  /**
 * Cursor for pagination (address of the last item from previous page)
 */
cursor: Option<string>;
  /**
 * Maximum number of items to return per page
 */
limit: u32;
}


/**
 * Pagination result with metadata for efficient navigation.
 * 
 * Contains the paginated data along with pagination metadata
 * to enable cursor-based navigation.
 */
export interface PaginatedLightProfiles {
  /**
 * The paginated data items
 */
data: Array<LightProfile>;
  /**
 * Whether there are more pages available
 */
has_more: boolean;
  /**
 * Cursor for the next page (None if this is the last page)
 */
next_cursor: Option<string>;
  /**
 * Total count of items matching the filter (optional, may be expensive to compute)
 */
total_count: Option<u32>;
}

/**
 * Storage keys for different data types in the user management contract.
 * 
 * This enum defines the various keys used to store and retrieve
 * user data from the contract's persistent storage.
 */
export type DataKey = {tag: "UserProfile", values: readonly [string]} | {tag: "Admin", values: readonly [string]} | {tag: "UserProfileLight", values: readonly [string]} | {tag: "UsersIndex", values: void} | {tag: "EmailIndex", values: readonly [string]} | {tag: "Admins", values: void} | {tag: "UserRole", values: readonly [string]} | {tag: "AdminConfig", values: void} | {tag: "RateLimit", values: readonly [string]} | {tag: "RolePermissions", values: readonly [UserRole]} | {tag: "UserPermissions", values: readonly [string]} | {tag: "DefaultRolePermissions", values: void};

export interface Client {
  /**
   * Construct and simulate a get_user_profile transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Retrieve a user profile for the authenticated user.
   * 
   * This function fetches the complete user profile associated with the provided
   * blockchain address. The user must be authenticated; otherwise, the function
   * will panic.
   * 
   * ### Arguments
   * 
   * * `env` - The Soroban environment.
   * * `user` - The address of the user whose profile is being reques
...[truncated for brevity]...