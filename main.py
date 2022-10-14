import json
import os

import rust
from solana.keypair import Keypair
from solana.publickey import PublicKey
from spl.token.instructions import create_associated_token_account, get_associated_token_address


key_path = os.path.join(os.path.expanduser('~'), ".config", "solana", "id.json")

with open(key_path, 'r') as keyfile:
    secret_key = json.load(keyfile)
    keypair = Keypair.from_secret_key(secret_key)
    signer = keypair.public_key

mint_account = Keypair.generate()
associated_token_address = get_associated_token_address(signer, mint_account.public_key)

# print(bytes(associated_token_address))
# data = {
#         "uri" : "www.google.com",
#         "associate_address" : bytes(str(associated_token_address), 'utf-8')
#     }

uri = "www.google.com"

print(PublicKey("6sDueq754X8Pm1bb5ubSYHW7xEPKPxD9BxoZksENqAke"))
print(list(bytes(PublicKey("6sDueq754X8Pm1bb5ubSYHW7xEPKPxD9BxoZksENqAke"))))

# print(signer.to_base58())
# print(signer.to_base58().decode())
# print(type(PUBKEY.build(bytes(signer))))
# print(list(bytes(assoc)))
rust.send_transaction(uri, (bytes(signer)))
