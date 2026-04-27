// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract TSCPAnchor {
    mapping(bytes32 => bool) public anchored;

    event BatchAnchored(bytes32 indexed batchHash, address indexed committer);

    function commit(bytes32 batchHash) external {
        require(!anchored[batchHash], "DUPLICATE_BATCH");
        anchored[batchHash] = true;
        emit BatchAnchored(batchHash, msg.sender);
    }

    function isAnchored(bytes32 batchHash) external view returns (bool) {
        return anchored[batchHash];
    }
}
