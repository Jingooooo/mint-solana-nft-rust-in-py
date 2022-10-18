import json
import os

import rust
from solana.keypair import Keypair
from solana.publickey import PublicKey
from solana.rpc.api import Client
from solders.signature import Signature
from spl.token.instructions import create_associated_token_account, get_associated_token_address


rpc_url = "https://api.devnet.solana.com"
key_path = os.path.join(os.path.expanduser('~'), ".config", "solana", "id.json")

with open(key_path, 'r') as keyfile:
    secret_key = json.load(keyfile)
    keypair = Keypair.from_secret_key(secret_key)
    signer = keypair.public_key

temp_account = Keypair.generate()
# associated_token_address = get_associated_token_address(signer, mint_account.public_key)

# print("secret", secret_key)
# print(bytes(associated_token_address))
# data = {
#         "uri" : "www.google.com",
#         "associate_address" : bytes(str(associated_token_address), 'utf-8')
#     }

uri = "www.google.com"

# print(PublicKey("6sDueq754X8Pm1bb5ubSYHW7xEPKPxD9BxoZksENqAke"))
# print(list(bytes(PublicKey("7Ew4GGk5pVbnwXbxT3UyeZb8BMSsg8oS4CpTcTy8Mv4f"))))


to_addr = PublicKey("EsL9DrUXjJhhFRQr1W5ouNeduPhC826vUTd8DDFD7Gsd")
client = Client(rpc_url)

for i in range(3):
    signature = rust.mint_and_freeze(rpc_url, secret_key, (bytes(to_addr)), uri)
    print(signature)
    tx = client.confirm_transaction(signature)
    # tx = client.confirm_transaction("5WY5r9vvFdxKV9VkX7AkaDtWzU6Yq5AooL4MY4hBQ6VvFjxSyGUrvgytgS6puWV6gybNeuioxg2Wzw4NipG9vDbw")
    print(json.dumps(tx, indent=4))
