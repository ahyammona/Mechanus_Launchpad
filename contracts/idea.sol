// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract AIAgentMemory {
    struct Idea {
        uint256 id;
        string content;     // the idea text (same one posted to Twitter)
        uint256 timestamp;
    }

    Idea[] public ideas;

    event IdeaStored(uint256 id, string content, uint256 timestamp);

    function storeIdea(string memory content) external {
        uint256 id = ideas.length;
        ideas.push(Idea(id, content, block.timestamp));
        emit IdeaStored(id, content, block.timestamp);
    }

    function latestIdea() external view returns (Idea memory) {
        require(ideas.length > 0, "No ideas stored yet");
        return ideas[ideas.length - 1];
    }

    function getIdea(uint256 id) external view returns (Idea memory) {
        require(id < ideas.length, "Invalid id");
        return ideas[id];
    }

    function totalIdeas() external view returns (uint256) {
        return ideas.length;
    }
}
