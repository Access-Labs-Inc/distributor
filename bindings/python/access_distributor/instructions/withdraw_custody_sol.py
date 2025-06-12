from __future__ import annotations
import typing
from solders.pubkey import Pubkey
from solders.instruction import Instruction, AccountMeta
import borsh_construct as borsh
from ..program_id import PROGRAM_ID


class WithdrawCustodySolArgs(typing.TypedDict):
    amount_lamports: int


layout = borsh.CStruct("amount_lamports" / borsh.U64)


class WithdrawCustodySolAccounts(typing.TypedDict):
    sol_custody: Pubkey
    owner: Pubkey


def withdraw_custody_sol(
    args: WithdrawCustodySolArgs,
    accounts: WithdrawCustodySolAccounts,
    program_id: Pubkey = PROGRAM_ID,
    remaining_accounts: typing.Optional[typing.List[AccountMeta]] = None,
) -> Instruction:
    keys: list[AccountMeta] = [
        AccountMeta(pubkey=accounts["sol_custody"], is_signer=False, is_writable=True),
        AccountMeta(pubkey=accounts["owner"], is_signer=True, is_writable=True),
    ]
    if remaining_accounts is not None:
        keys += remaining_accounts
    identifier = b"v\x86T';|\xd9\x9a"
    encoded_args = layout.build(
        {
            "amount_lamports": args["amount_lamports"],
        }
    )
    data = identifier + encoded_args
    return Instruction(program_id, data, keys)
