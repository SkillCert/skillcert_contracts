// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract UserProfile {
    struct Profile {
        string name;
        string email;
        string role;
        string country;
        string profilePicture;
        string language;
        string metadata;
        bool exists;
    }

    // Mapping from user address to their Profile
    mapping(address => Profile) public profiles;

    // Mapping to ensure email uniqueness
    mapping(string => bool) private emailExists;

    // Event emitted when a profile is created
    event ProfileCreated(
        address indexed userAddress,
        string name,
        string email,
        string role,
        string country,
        string profilePicture,
        string language,
        string metadata
    );

    /**
     * @dev Creates a new user profile.
     * @param _name The name of the user.
     * @param _email The email of the user.
     * @param _role The role of the user.
     * @param _country The country of the user.
     * @param _metadata Optional metadata.
     */
    function createProfile(
        string calldata _name,
        string calldata _email,
        string calldata _role,
        string calldata _country,
        string calldata _metadata
    ) external {
        // Validate required fields
        require(bytes(_name).length > 0, "Name is required");
        require(bytes(_email).length > 0, "Email is required");
        require(bytes(_role).length > 0, "Role is required");

        // Ensure the user does not already have a profile
        require(!profiles[msg.sender].exists, "Profile already exists");

        // Ensure email uniqueness
        require(!emailExists[_email], "Email already in use");

        // Assign default values
        string memory defaultProfilePicture = "https://example.com/default-avatar.png";
        string memory defaultLanguage = "en";

        // Store the profile
        profiles[msg.sender] = Profile({
            name: _name,
            email: _email,
            role: _role,
            country: _country,
            profilePicture: defaultProfilePicture,
            language: defaultLanguage,
            metadata: _metadata,
            exists: true
        });

        // Mark email as used
        emailExists[_email] = true;

        // Emit event
        emit ProfileCreated(
            msg.sender,
            _name,
            _email,
            _role,
            _country,
            defaultProfilePicture,
            defaultLanguage,
            _metadata
        );
    }

    /**
     * @dev Checks if an email is already registered.
     */
    function isEmailRegistered(string calldata _email) external view returns (bool) {
        return emailExists[_email];
    }

    /**
     * @dev Returns the profile of a user.
     */
    function getProfile(address _user) external view returns (
        string memory name,
        string memory email,
        string memory role,
        string memory country,
        string memory profilePicture,
        string memory language,
        string memory metadata
    ) {
        require(profiles[_user].exists, "Profile does not exist");
        Profile memory p = profiles[_user];
        return (p.name, p.email, p.role, p.country, p.profilePicture, p.language, p.metadata);
    }
}
