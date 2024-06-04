from OpenSSL import crypto
from datetime import datetime
import jks

my_keystore: jks.KeyStore = jks.KeyStore.load(r'content/my_keystore.jks', 'admin')

def display_keystore_information(
    my_keystore: jks.KeyStore
) -> None:
    for alias, entry in my_keystore.entries.items():
        x509: crypto.X509 = crypto.load_certificate(crypto.FILETYPE_ASN1, entry.cert)
        subject: crypto.X509Name = x509.get_subject()
        issuer: crypto.X509Name = x509.get_issuer()
        is_self_signed: bool = subject == issuer
        not_before: str = datetime.strptime(x509.get_notBefore().decode('ascii'), '%Y%m%d%H%M%SZ')
        not_after: str = datetime.strptime(x509.get_notAfter().decode('ascii'), '%Y%m%d%H%M%SZ')

        print(f'Alias: {alias}')
        print(f'  Proprietário do Certificado: {subject.CN}')
        print(f'  Emissor do Certificado: {issuer.CN}')
        print(f'  Auto-assinado: {is_self_signed}')
        print(f'  Período de Validade: {not_before} - {not_after}')
        print('')

if (__name__ == "__main__"):
    display_keystore_information(my_keystore)
