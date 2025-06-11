from __future__ import annotations
import typing
from solders.pubkey import Pubkey
from solders.instruction import Instruction, AccountMeta
from ..program_id import PROGRAM_ID


class SetAdminAccounts(typing.TypedDict):
    distributor: Pubkey
    admin: Pubkey
    new_admin: Pubkey


def set_admin(
    accounts: SetAdminAccounts,
    program_id: Pubkey = PROGRAM_ID,
    remaining_accounts: typing.Optional[typing.List[AccountMeta]] = None,
) -> Instruction:
    keys: list[AccountMeta] = [
        AccountMeta(pubkey=accounts["distributor"], is_signer=False, is_writable=True),
        AccountMeta(pubkey=accounts["admin"], is_signer=True, is_writable=True),
        AccountMeta(pubkey=accounts["new_admin"], is_signer=False, is_writable=True),
    ]
    if remaining_accounts is not None:
        keys += remaining_accounts
    identifier = b"\xfb\xa3\x004[\xc2\xbb\\"
    encoded_args = b""
    data = identifier + encoded_args
    return Instruction(program_id, data, keys)
