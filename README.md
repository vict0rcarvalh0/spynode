# spynode
Setting up a node that listens to transactions but doesn’t participate in block building

Problem Statement: Simulate/Impersonate/Mimic Validator node to get block data and then forward it to RPC because RPC gets data slower 

Roadmap:
- Isolate Logic to run Node(standalone) from agave codebase or from here
- Where does the validator communicate with gossip? 
- Remove any voting/block building sections 
- Forward Data to RPCs 
