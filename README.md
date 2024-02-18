# Run the code : Instruction(cluster-localnet)
1. clone repo.
2. npm install
3. add the address in clamespl.js file.
4. anchor test (this command will build and deplot the contract and run the testcase:-clamespl.js ).

# Structure to store whitelist address and flows:

BaseAccount Struct: This struct contains two fields:

1. total_add: Tracks the total number of addresses stored in the account.
add_list: A vector of ItemStruct objects. Each ItemStruct represents a user address (user_address) and an associated token amount (spl_token_amt).
ItemStruct: This struct defines the structure of each item stored in the add_list. It contains two fields:

2. spl_token_amt: The amount of SPL tokens associated with the user address.
user_address: The public key representing the user's address.
add_address Function: This function allows adding a new address along with its associated token amount to the BaseAccount. 
It takes the SPL token amount (spl_token_amt) and the user's address (user_address) as parameters.
It creates a new ItemStruct with these values and appends it to the add_list vector.

3. check Function: This function checks if a given user address matches any of the stored addresses in the BaseAccount. 
It first retrieves the caller's public key (caller_pubkey). Then, it iterates through the add_list vector and checks 
if any ItemStruct's user_address matches the caller's public key. If a match is found, it returns the 
associated token amounts for that user. Otherwise, it returns a custom error indicating that the caller is not authorized.

4. The filtering of addresses is done using the iter() and filter() methods on the add_list vector.
These methods iterate through the vector and apply a predicate to each item.
The predicate checks if the user_address field of each ItemStruct matches the caller's public key. 
If a match is found, the associated token amount (spl_token_amt) is collected into a vector using the map() method.

Overall, the add_address function adds new addresses to the BaseAccount, while the check function verifies whether 
a given address is stored in the account and returns the associated token amounts if it is found.

# Here , we have 2 functions 1. check 2. transfer-spl 
We handle from web3 , that's when user(whitelisted adddress) call the function for clame spl token , 
1st it will check the validation for the address is whitelisted or not and if caller is whitelist address , then the transfer function call Otherwise throw error.
