from OpenSSL import crypto
import jks

my_keystore: jks.KeyStore = jks.KeyStore.load(r'content/my_keystore.jks', 'admin')

for alias, entry in my_keystore.entries.items():
    if isinstance(entry, jks.PrivateKeyEntry):
        for certificate in entry.cert_chain:
            x509: crypto.X509 = crypto.load_certificate(crypto.FILETYPE_ASN1, certificate.cert)
            subject: crypto.X509Name = x509.get_subject()
            issuer: crypto.X509Name = x509.get_issuer()
            is_self_signed: bool = subject == issuer
            not_before: str = x509.get_notBefore().decode('ascii')
            not_after: str = x509.get_notAfter().decode('ascii')

            print(f'Alias: {alias}')
            print(f'  Proprietário do Certificado: {subject.CN}')
            print(f'  Emissor do Certificado: {issuer.CN}')
            print(f'  Auto-assinado: {is_self_signed}')
            print(f'  Período de Validade: {not_before} - {not_after}')
            print('')
