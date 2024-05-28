from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.hazmat.primitives import hashes
from os import path
from shutil import copy

def generate_rsa_keys() -> tuple[rsa.RSAPrivateKey, rsa.RSAPublicKey]:
    private_key: rsa.RSAPrivateKey = rsa.generate_private_key(
        public_exponent=65537,
        key_size=2048
    )
    public_key: rsa.RSAPublicKey = private_key.public_key()

    return private_key, public_key

def get_file_path_from_user() -> str:
    return input('Enter the complete path to the file to be signed: ')

def upload_file(
    file_path: str
) -> str:
    file_name: str = path.basename(file_path)
    destination_path: str = path.join('content', file_name)

    if (path.exists(destination_path)):
        return destination_path

    copy(file_path, destination_path)
    return destination_path

def apply_digital_signature(
    private_key_a: rsa.RSAPrivateKey,
    destination_path: str
) -> str:
    with open(destination_path, 'rb') as data:
        file_data: bytes = data.read()
    
    digital_signature: bytes = private_key_a.sign(
        file_data,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.MAX_LENGTH
        ),
        hashes.SHA256()
    )
    signed_file_path: str = 'content/signed.bin'

    with open(signed_file_path, 'wb') as data:
        data.write(digital_signature)

    return signed_file_path

def verify_signature(
    public_key: rsa.RSAPublicKey,
    destination_path: str,
    signature_path: str
) -> None:
    with open(destination_path, 'rb') as data:
        file_data: bytes = data.read()

    with open(signature_path, 'rb') as data:
        signature_data: bytes = data.read()

    try:
        public_key.verify(
            signature_data,
            file_data,
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.MAX_LENGTH
            ),
            hashes.SHA256()
        )
        print('Valid signature.')
    except Exception as _:
        print('Invalid signature.')

if (__name__ == "__main__"):
    (private_key_a, public_key_a) = generate_rsa_keys()
    (_, public_key_b) = generate_rsa_keys()

    file_path: str = get_file_path_from_user()

    if path.exists(file_path):
        destination_path: str = upload_file(file_path)
        signature_path: str = apply_digital_signature(
            private_key_a,
            destination_path
        )
        verify_signature(
            public_key_a,
            destination_path,
            signature_path
        )
        verify_signature(
            public_key_b,
            destination_path,
            signature_path
        )
    else:
        raise Exception('File path does not exists.')
