from Crypto.Cipher import Blowfish
from Crypto import Cipher
from Crypto.Util.Padding import pad, unpad
from Crypto.Random import get_random_bytes
import binascii
import os

block_filling_scheme: str = 'pkcs7' # Equivalent to 'PKCS#5'
my_key: bytes = b'ABCDE'
input_file_path: str = r'content/input_file_blowfish_exercises.pdf'
output_file_path: str = r'content/saida.bin'
output_deciphered_file_path: str = r'content/decriptografado.pdf'

def execute_cases(
   block_filling_scheme: str,
   my_key: bytes,
   input_file_path: str,
   output_file_path: str
) -> None:
    random_bytes_initialization_vector: bytes = get_random_bytes(Blowfish.block_size)
    blowfish_ecb_cipher: Cipher = Blowfish.new(my_key, Blowfish.MODE_ECB)
    blowfish_cbc_cipher: Cipher = Blowfish.new(my_key, Blowfish.MODE_CBC, random_bytes_initialization_vector)
    blowfish_cbc_cipher_with_bytes: Cipher = Blowfish.new(my_key, Blowfish.MODE_CBC, bytes([1, 1, 2, 2, 3, 3, 4, 4]))
    blowfish_cbc_cipher_with_bytes_2: Cipher = Blowfish.new(my_key, Blowfish.MODE_CBC, bytes([10, 20, 30, 40, 50, 60, 70, 80]))

    # First Case
    print("\nFirst Case")
    simple_blowfish_ecb_ciphering_with_text(
        'FURB',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Second Case
    print("\nSecond Case")
    simple_blowfish_ecb_ciphering_with_text(
        'COMPUTADOR',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Third Case
    print("\nThird Case")
    simple_blowfish_ecb_ciphering_with_text(
        'SABONETE',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Fourth Case
    print("\nFourth Case")
    simple_blowfish_ecb_ciphering_with_bytes(
        bytes([8, 8, 8, 8, 8, 8, 8, 8]),
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Fifth Case
    print("\nFifth Case")
    simple_blowfish_ecb_ciphering_with_text(
        'SABONETESABONETESABONETE',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Sixth Case
    print("\nSixth Case")
    ciphered_sixth_case_plaintext: bytes = simple_blowfish_cbc_ciphering_with_text(
        'FURB',
        blowfish_cbc_cipher,
        block_filling_scheme
    )

    simple_blowfish_cbc_deciphering_with_text(
        ciphered_sixth_case_plaintext,
        Blowfish.new(my_key, Blowfish.MODE_CBC, random_bytes_initialization_vector),
        block_filling_scheme
    )

    # Seventh Case
    print("\nSeventh Case")
    simple_blowfish_cbc_ciphering_with_text(
        'FURB',
        blowfish_cbc_cipher_with_bytes,
        block_filling_scheme
    )

    # Eighth Case
    print("\nEighth Case")
    simple_blowfish_cbc_ciphering_with_text(
        'SABONETESABONETESABONETE',
        blowfish_cbc_cipher_with_bytes,
        block_filling_scheme
    )

    # Ninth Case
    print("\nNinth Case")
    ciphered_ninth_case_plaintext: bytes = simple_blowfish_cbc_ciphering_with_text(
        'SABONETESABONETESABONETE',
        blowfish_cbc_cipher_with_bytes_2,
        block_filling_scheme
    )

    simple_blowfish_cbc_deciphering_with_text(
        ciphered_ninth_case_plaintext,
        Blowfish.new(my_key, Blowfish.MODE_CBC, bytes([1, 1, 2, 2, 3, 3, 4, 4])),
        block_filling_scheme
    )

    # Tenth Case
    print("\nTenth Case")
    _ = simple_blowfish_ecb_ciphering_with_text(
        'FURB',
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    '''
    simple_blowfish_ecb_deciphering_with_text(
        ciphered_tenth_case_plaintext,
        Blowfish.new('11111', Blowfish.MODE_ECB),
        block_filling_scheme
    )
    '''

    # Eleventh Case
    print("\nEleventh Case")
    cipher_pdf_with_ecb(
        input_file_path,
        output_file_path,
        blowfish_ecb_cipher,
        block_filling_scheme
    )

    # Twelfth Case
    print("\nTwelfth Case")
    decipher_pdf_with_ecb(
        output_file_path,
        output_deciphered_file_path,
        Blowfish.new(my_key, Blowfish.MODE_ECB),
        block_filling_scheme
    )
    print("Generated: ", output_deciphered_file_path)

def simple_blowfish_ecb_ciphering_with_text(
    case_text: str,
    blowfish_ecb_cipher: Cipher,
    block_filling_scheme: str
) -> bytes:
    plaintext: bytes = pad(
        data_to_pad=case_text.encode(),
        block_size=Blowfish.block_size,
        style=block_filling_scheme
    )

    ciphered_case_plaintext: bytes = blowfish_ecb_cipher.encrypt(plaintext)
    print("\nExecuing Blowfish with ECB..")
    print("Case Text: ", case_text)
    print("Case Result in Hex: ", binascii.hexlify(ciphered_case_plaintext))
    print("Case Result Extension: ", len(ciphered_case_plaintext))

    return ciphered_case_plaintext

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

    ciphered_case_plainbytes: bytes = blowfish_ecb_cipher.encrypt(plainbytes)
    print("\nExecuing Blowfish with ECB..")
    print("Case Byte Sequence: ", case_byte_sequence)
    print("Case Result: ", ciphered_case_plainbytes)
    print("Case Result in Hex: ", binascii.hexlify(ciphered_case_plainbytes))

def simple_blowfish_ecb_deciphering_with_text(
    ciphered_case_plaintext: bytes,
    blowfish_ecb_cipher: Cipher,
    block_filling_scheme: str
) -> None:
    deciphered_case_plaintext: bytes = blowfish_ecb_cipher.decrypt(ciphered_case_plaintext)
    deciphered_case_plaintext_unpad: bytes = unpad(
        padded_data=deciphered_case_plaintext,
        block_size=Blowfish.block_size,
        style=block_filling_scheme
    )

    print("\nExecuting Blowfish with ECB..")
    print("Case Deciphered Text: ", deciphered_case_plaintext_unpad)
    print("Case Result in Hex: ", binascii.hexlify(deciphered_case_plaintext_unpad))

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

    ciphered_case_plaintext: bytes = blowfish_cbc_cipher.encrypt(plaintext)
    print("\nExecuing Blowfish with CBC..")
    print("Case Text: ", case_text)
    print("Case Result in Hex: ", binascii.hexlify(ciphered_case_plaintext))

    return ciphered_case_plaintext

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
    print("Case Deciphered Text: ", deciphered_case_plaintext_unpad)
    print("Case Result in Hex: ", binascii.hexlify(deciphered_case_plaintext_unpad))

def cipher_pdf_with_ecb(
    input_file_path: str,
    output_file_path: str,
    blowfish_ecb_cipher: Cipher,
    block_filling_scheme: str
) -> None:
    with open(input_file_path, 'rb') as data:
        pdf_data: bytes = data.read()
    
    ciphered_data: bytes = blowfish_ecb_cipher.encrypt(
        pad(
            data_to_pad=pdf_data,
            block_size=Blowfish.block_size,
            style=block_filling_scheme
        )
    )

    with open(output_file_path, 'wb') as data:
        data.write(ciphered_data)
    
    input_file_size = os.path.getsize(input_file_path)
    output_file_size = os.path.getsize(output_file_path)

    print("Size of the first archive: ", input_file_size)
    print("Size of the second archive: ", output_file_size)

def decipher_pdf_with_ecb(
    output_file_path: str,
    output_deciphered_file_path: str,
    blowfish_ecb_cipher: Cipher,
    block_filling_scheme: str
) -> None:
    with open(output_file_path, 'rb') as data:
        ciphered_data: bytes = data.read()

    deciphered_data: bytes = blowfish_ecb_cipher.decrypt(ciphered_data)
    deciphered_plaintext: bytes = unpad(
        padded_data=deciphered_data,
        block_size=Blowfish.block_size,
        style=block_filling_scheme
    )

    with open(output_deciphered_file_path, 'wb') as data:
        data.write(deciphered_plaintext)

execute_cases(
    block_filling_scheme,
    my_key,
    input_file_path,
    output_file_path
)
