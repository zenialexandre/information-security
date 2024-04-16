from Crypto.Cipher import Blowfish
from Crypto import Cipher
from Crypto.Util.Padding import pad, unpad
from Crypto.Random import get_random_bytes
import binascii

block_filling_scheme: str = 'pkcs7' # Equivalent to 'PKCS#5'
my_key: bytes = b'ABCDE'

def execute_cases(
   block_filling_scheme: str,
   my_key: bytes
) -> None:
    blowfish_ecb_cipher: Cipher = Blowfish.new(my_key, Blowfish.MODE_ECB)
    blowfish_cbc_cipher: Cipher = Blowfish.new(my_key, Blowfish.MODE_CBC, get_random_bytes(Blowfish.block_size))

    # First Case
    simple_blowfish_ecb_ciphering_with_text(
        'FURB',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Second Case
    simple_blowfish_ecb_ciphering_with_text(
        'COMPUTADOR',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Third Case
    simple_blowfish_ecb_ciphering_with_text(
        'SABONETE',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Fourth Case
    simple_blowfish_ecb_ciphering_with_bytes(
        bytes([8, 8, 8, 8, 8, 8, 8, 8]),
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Fifth Case
    simple_blowfish_ecb_ciphering_with_text(
        'SABONETESABONETESABONETE',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Sixth Case
    enciphered_text: bytes = simple_blowfish_cbc_ciphering_with_text(
        'FURB',
        blowfish_cbc_cipher,
        block_filling_scheme
    )

    simple_blowfish_cbc_deciphering_with_text(
        enciphered_text,
        blowfish_cbc_cipher,
        block_filling_scheme
    )

def simple_blowfish_ecb_ciphering_with_text(
    case_text: str,
    blowfish_ecb_cipher: Cipher,
    block_filling_scheme: str
) -> None:
    plaintext: bytes = pad(
        data_to_pad=case_text.encode(),
        block_size=Blowfish.block_size,
        style=block_filling_scheme
    )

    enciphered_case_plaintext: bytes = blowfish_ecb_cipher.encrypt(plaintext)
    print("\nExecuing Blowfish with ECB..")
    print("Case Text: ", case_text)
    print("Case Result in Hex: ", binascii.hexlify(enciphered_case_plaintext))
    print("Case Result Extension: ", len(enciphered_case_plaintext))

def simple_blowfish_ecb_ciphering_with_bytes(
    case_byte_sequence: bytes,
    blowfish_ecb_cipher: Cipher,
    block_filling_scheme: str
) -> None:
    plainbytes: bytes = pad(
        data_to_pad=case_byte_sequence,
        block_size=Blowfish.block_size,
        style=block_filling_scheme
    )

    enciphered_case_plainbytes: bytes = blowfish_ecb_cipher.encrypt(plainbytes)
    print("\nExecuing Blowfish with ECB..")
    print("Case Byte Sequence: ", case_byte_sequence)
    print("Case Result: ", enciphered_case_plainbytes)
    print("Case Result in Hex: ", binascii.hexlify(enciphered_case_plainbytes))

def simple_blowfish_cbc_ciphering_with_text(
    case_text: str,
    blowfish_cbc_cipher: Cipher,
    block_filling_scheme: str
) -> bytes:
    plaintext: bytes = pad(
        data_to_pad=case_text.encode(),
        block_size=Blowfish.block_size,
        style=block_filling_scheme
    )

    enciphered_case_plaintext: bytes = blowfish_cbc_cipher.encrypt(plaintext)
    print("\nExecuing Blowfish with CBC..")
    print("Case Text: ", case_text)
    print("Case Result in Hex: ", binascii.hexlify(enciphered_case_plaintext))

    return enciphered_case_plaintext

def simple_blowfish_cbc_deciphering_with_text(
    ciphered_case_plaintext: bytes,
    blowfish_cbc_cipher: Cipher,
    block_filling_scheme: str
) -> None:
    deciphered_case_plaintext: str = blowfish_cbc_cipher.decrypt(ciphered_case_plaintext)
    deciphered_case_plaintext_unpad: bytes = unpad(
        padded_data=deciphered_case_plaintext,
        block_size=Blowfish.block_size,
        style=block_filling_scheme
    )

    print("\nExecuting Blowfish with CBC..")
    print("Case Ciphered Text: ", ciphered_case_plaintext)
    print("Case Result in Hex: ", binascii.hexlify(deciphered_case_plaintext_unpad))

execute_cases(block_filling_scheme, my_key)
