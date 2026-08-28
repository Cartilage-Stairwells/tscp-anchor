// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract TSCPAnchor {
    mapping(bytes32 => bool) public anchored;
    mapping(address => bool) public authorizedCommitters;
    address public owner;

    event BatchAnchored(bytes32 indexed batchHash, address indexed committer);
    event CommitterAuthorized(address indexed committer);
    event CommitterRevoked(address indexed committer);

    modifier onlyOwner() { require(msg.sender == owner, "TSCP: not owner"); _; }
    modifier onlyAuthorized() { require(authorizedCommitters[msg.sender], "TSCP: not authorized committer"); _; }

    constructor() {
        owner = msg.sender;
    }

    function authorizeCommitter(address committer) external onlyOwner {
        authorizedCommitters[committer] = true;
        emit CommitterAuthorized(committer);
    }

    function revokeCommitter(address committer) external onlyOwner {
        authorizedCommitters[committer] = false;
        emit CommitterRevoked(committer);
    }

    function commit(bytes32 batchHash) external onlyAuthorized {
        require(!anchored[batchHash], "DUPLICATE_BATCH");
        anchored[batchHash] = true;
        emit BatchAnchored(batchHash, msg.sender);
    }

    function isAnchored(bytes32 batchHash) external view returns (bool) {
        return anchored[batchHash];
    }
}
