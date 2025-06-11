from solders.pubkey import Pubkey


# get the distributor associated token address
def get_distributor_pda(mint, program_id, creator, version=0):
    (distributor, bump) = Pubkey.find_program_address(
        [
            b"MerkleDistributor", 
            bytes(mint), 
            bytes(creator), 
            version.to_bytes(8, "little")
        ], 
        program_id
    )
    return distributor, bump
