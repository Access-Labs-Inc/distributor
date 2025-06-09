from typing import List

from solders.account import Account
from solders.bankrun import ProgramTestContext
from solders.instruction import Instruction
from solders.message import Message
from solders.token.associated import get_associated_token_address
from dataclasses import dataclass
from time import time

from client_py.instructions.new_distributor import new_distributor
from merkle_tree import MerkleTree
from test_utils import get_distributor_pda
from client_py.instructions.clawback import clawback
from solders.clock import Clock

from solders.transaction import TransactionError, VersionedTransaction
from client_py.program_id import PROGRAM_ID
from solders.token.state import TokenAccount, TokenAccountState, Mint
from spl.token.instructions import create_associated_token_account

from pathlib import Path
from pytest import mark, raises
from solders.bankrun import start_anchor
from solders.pubkey import Pubkey
from solders.keypair import Keypair

TOKEN_PROGRAM_ID = Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")


@dataclass
class TestHook:
    mint: Pubkey
    clawback_keypair: Keypair
    distributor_ata: Pubkey
    program_id: Pubkey
    context: ProgramTestContext


async def init_test_accounts(program_id) -> TestHook:
    mint_address = Pubkey.new_unique()
    clawback_keypair = Keypair()

    clawback_token_acc = TokenAccount(
        mint=mint_address,
        owner=clawback_keypair.pubkey(),
        amount=1_000,
        delegate=None,
        state=TokenAccountState.Initialized,
        is_native=None,
        delegated_amount=0,
        close_authority=None,
    )

    accounts = [
        (
            get_associated_token_address(clawback_keypair.pubkey(), mint_address),
            Account(
                lamports=1_000_000_000,
                data=bytes(clawback_token_acc),
                owner=TOKEN_PROGRAM_ID,
                executable=False,
            ),
        ),
        (
            mint_address,
            Account(
                data=bytes(
                    Mint(
                        decimals=9,
                        mint_authority=None,
                        supply=10000,
                        is_initialized=True,
                    )
                ),
                lamports=100_000,
                owner=TOKEN_PROGRAM_ID,
            ),
        ),
    ]

    context = await start_anchor(Path("../"), accounts=accounts)

    payer = context.payer
    distributor = get_distributor_pda(mint_address,  program_id, payer.pubkey(), 0)[0]
    distributor_ata = get_associated_token_address(distributor, mint_address)

    return TestHook(
        mint_address, clawback_keypair, distributor_ata, program_id, context
    )


@mark.asyncio
async def test_new_distributor():
    """Test that a new distributor can successfully be created"""
    testhook = await init_test_accounts(PROGRAM_ID)
    context = testhook.context
    program_id = testhook.program_id
    mint = testhook.mint

    payer = context.payer 
    (distributor, bump) = get_distributor_pda(mint, program_id, payer.pubkey(), 0)
    ata_creation_ix = create_associated_token_account(
        payer=payer.pubkey(),
        owner=distributor,
        mint=mint
    )   

    curr_ts = await context.banks_client.get_clock()
    distributor = new_distributor(
        {
            "version": 0,
            "root": [0] * 32,
            "max_total_claim": 100_00_00,
            "max_num_nodes": 1,
            "start_vesting_ts": curr_ts.unix_timestamp + 100000,
            "end_vesting_ts": curr_ts.unix_timestamp + 200000,
        },
        {
            "distributor": distributor,
            "token_vault": get_associated_token_address(distributor, mint),
            "mint": mint,
            "creator": payer.pubkey(),
        },
    )

    ixs = [ata_creation_ix, distributor]
    blockhash = context.last_blockhash
    msg = Message.new_with_blockhash(ixs, payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    client = context.banks_client
    await client.process_transaction(tx)


@mark.asyncio
async def test_clawback():
    """Test that an account can be clawed back successfully"""
    test_hook = await init_test_accounts(PROGRAM_ID)
    context = test_hook.context
    program_id = test_hook.program_id
    mint = test_hook.mint
    clawback_address = get_associated_token_address(
        test_hook.clawback_keypair.pubkey(), mint
    )
    distributor_ata = test_hook.distributor_ata

    # setup distributor
    payer = context.payer
    (distributor, bump) = get_distributor_pda(mint, program_id, payer.pubkey(), 0)
    ata_creation_ix = create_associated_token_account(
        payer=payer.pubkey(),
        owner=distributor,
        mint=mint
    )
    
    curr_ts = await context.banks_client.get_clock()
    new_distributor_ix = new_distributor(
        {
            "version": 0,
            "root": [0] * 32,
            "max_total_claim": 100_00_00,
            "max_num_nodes": 1,
            "start_vesting_ts": curr_ts.unix_timestamp + 100000,
            "end_vesting_ts": curr_ts.unix_timestamp + 200000,
        },
        {
            "distributor": distributor,
            "token_vault": get_associated_token_address(distributor, mint),
            "mint": mint,
            "creator": payer.pubkey(),
        },
    )

    clawback_ix = clawback(
        {
            "distributor": distributor,
            "from_": distributor_ata,
            "to": clawback_address,
            "admin": payer.pubkey(),
        }
    )

    ixs = [ata_creation_ix, new_distributor_ix, clawback_ix]
    blockhash = context.last_blockhash
    msg = Message.new_with_blockhash(ixs, payer.pubkey(), blockhash)
    tx = VersionedTransaction(msg, [payer])
    client = context.banks_client
    await client.process_transaction(tx)


async def setup_clawback_test_case() -> (TestHook, List[Instruction]):
    """Setup a test case for clawback by initializing a new distributor"""
    test_hook = await init_test_accounts(PROGRAM_ID)
    context = test_hook.context
    program_id = test_hook.program_id
    mint = test_hook.mint

    # setup distributor
    payer = context.payer
    (distributor, bump) = get_distributor_pda(mint, program_id, payer.pubkey(), 0)
    curr_ts = int(time())

    new_distributor_ix = new_distributor(
        {
            "version": 0,
            "root": [0] * 32,
            "max_total_claim": 100_00_00,
            "max_num_nodes": 1,
            "start_vesting_ts": curr_ts,
            "end_vesting_ts": curr_ts + 100_000,
        },
        {
            "distributor": distributor,
            "token_vault": get_associated_token_address(distributor, mint),
            "mint": mint,
            "creator": context.payer.pubkey(),
        },
    )

    return test_hook, new_distributor_ix

@mark.asyncio
def test_load_merkle_tree():
    """Test that loading the merkle tree works correctly"""
    path = "merkle_tree.json"
    with open(path, "r") as f:
        json_str = f.read()

    merkle_tree = MerkleTree.from_json(json_str)
    assert merkle_tree.max_total_claim == 600000000000

