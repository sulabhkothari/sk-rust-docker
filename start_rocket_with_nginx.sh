cd /app
#nginx
cargo run

# Create certificate from keychain->certificate assistant->create a certificate
# Export the certificate and set import password
# Use below commands to export crt and key files
# openssl pkcs12 -in /Users/sulabhkothari/Documents/sk-web-cert.p12 -clcerts -nokeys -out domain.crt
# openssl pkcs12 -in /Users/sulabhkothari/Documents/sk-web-cert.p12 -nocerts -nodes -out domain.rsa