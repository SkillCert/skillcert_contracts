const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("UserProfile", function () {
  let UserProfile;
  let userProfile;
  let owner;
  let addr1;
  let addr2;

  beforeEach(async function () {
    UserProfile = await ethers.getContractFactory("UserProfile");
    [owner, addr1, addr2] = await ethers.getSigners();
    userProfile = await UserProfile.deploy();
  });

  describe("Profile Creation", function () {
    it("Should create a profile successfully with valid inputs", async function () {
      await expect(
        userProfile.connect(addr1).createProfile(
          "Alice",
          "alice@example.com",
          "Developer",
          "Brazil",
          "some-metadata"
        )
      )
        .to.emit(userProfile, "ProfileCreated")
        .withArgs(
          addr1.address,
          "Alice",
          "alice@example.com",
          "Developer",
          "Brazil",
          "https://example.com/default-avatar.png",
          "en",
          "some-metadata"
        );

      const profile = await userProfile.getProfile(addr1.address);
      expect(profile.name).to.equal("Alice");
      expect(profile.email).to.equal("alice@example.com");
      expect(profile.role).to.equal("Developer");
      expect(profile.country).to.equal("Brazil");
      expect(profile.profilePicture).to.equal("https://example.com/default-avatar.png");
      expect(profile.language).to.equal("en");
      expect(profile.metadata).to.equal("some-metadata");
    });

    it("Should fail if required fields are missing", async function () {
      await expect(
        userProfile.connect(addr1).createProfile(
          "",
          "alice@example.com",
          "Developer",
          "Brazil",
          ""
        )
      ).to.be.revertedWith("Name is required");

      await expect(
        userProfile.connect(addr1).createProfile(
          "Alice",
          "",
          "Developer",
          "Brazil",
          ""
        )
      ).to.be.revertedWith("Email is required");

      await expect(
        userProfile.connect(addr1).createProfile(
          "Alice",
          "alice@example.com",
          "",
          "Brazil",
          ""
        )
      ).to.be.revertedWith("Role is required");
    });

    it("Should fail if email is already in use", async function () {
      await userProfile.connect(addr1).createProfile(
        "Alice",
        "alice@example.com",
        "Developer",
        "Brazil",
        ""
      );

      await expect(
        userProfile.connect(addr2).createProfile(
          "Bob",
          "alice@example.com",
          "Designer",
          "USA",
          ""
        )
      ).to.be.revertedWith("Email already in use");
    });

    it("Should fail if user already has a profile", async function () {
      await userProfile.connect(addr1).createProfile(
        "Alice",
        "alice@example.com",
        "Developer",
        "Brazil",
        ""
      );

      await expect(
        userProfile.connect(addr1).createProfile(
          "Alice 2",
          "alice2@example.com",
          "Manager",
          "Brazil",
          ""
        )
      ).to.be.revertedWith("Profile already exists");
    });
  });
});
