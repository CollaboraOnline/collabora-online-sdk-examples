#!/bin/bash

# This scripts philosophy: KISS
# Make it safe and easy to understand.

# See also:
#   https://github.com/CollaboraOnline/online/blob/fa9b763cf153f8869c7ff0118a123522f7878fae/test/WopiProofTests.cpp
#   https://sdk.collaboraonline.com/docs/advanced_integration.html#wopi-proof
#   https://learn.microsoft.com/en-us/microsoft-365/cloud-storage-partner-program/online/scenarios/proofkeys
#   https://learn.microsoft.com/en-us/microsoft-365/cloud-storage-partner-program/rest/common-headers
#   https://learn.microsoft.com/en-us/openspecs/office_protocols/ms-wopi/70254fa6-9d3d-4f93-9b1a-e6597a63b900
#   https://github.com/microsoft/Office-Online-Test-Tools-and-Documentation/blob/4c39bf7a801bc8ee7974926d318d7f60bb3e8e16/docs/online/src/discovery.rst

set -e

modulus_base_64="${1}"
exponent_base_64="${2}"
token="${3}"
timestamp_decimal="${4}"  # 100 nanoseconds since 0001-01-01T00:00 UTC
url="${5}"
proof_signature_base_64="${6}"

echo
echo


# See also:
#   https://github.com/microsoft/Office-Online-Test-Tools-and-Documentation/blob/e9f0cad077cbc0c62953ab120a2dc84568ae40d3/samples/SampleWopiHandler/SampleWopiHandler/ProofKeyHelper.cs#L29
#
use_example_data () {
  if [ -n "${modulus_base_64}" ]; then
    echo 'NOT RUNNING use_example_data !!!'
    echo
    return;
  fi
  echo 'Using example data from:'
  echo \
'https://github.com/microsoft/Office-Online-Test-Tools-and-Documentation/blob/e9f0cad077cbc0c62953ab120a2dc84568ae40d3/samples/SampleWopiHandler/SampleWopiHandler.UnitTests/ProofKeyTests.cs#L21'
  echo
  modulus_base_64=\
'0HOWUPFFgmSYHbLZZzdWO/HUOr8YNfx5NAl7GUytooHZ7B9QxQKTJpj0NIJ4XEskQW8e4dLzRrPbNOOJ+KpWHttXz8HoQXkkZV/gYNxaNHJ8/pRXGMZzfVM5vchhx/2C7ULPTrpBsSpmfWQ6ShaVoQzfThFUd0MsBvIN7HVtqzPx9jbSV04wAqyNjcro7F3iu9w7AEsMejHbFlWoN+J05dP5ixryF7+2U5RVmjMt7/dYUdCoiXvCMt2CaVr0XEG6udHU4iDKVKZjmUBc7cTWRzhqEL7lZ1yQfylp38Nd2xxVJ0sSU7OkC1bBDlePcYGaF3JjJgsmp/H5BNnlW9gSxQ==';
  exponent_base_64='AQAB'
  token=\
'RLoY%2f3D73%2fjwt6IQqR1wHqCEKDxRf2v0GPDa0ZHTlA6ik1%2fNBHDD6bHCI0BQrvacjNBL8ok%2fZsVPI%2beAIA5mHSOUbhW9ohowwD6Ljlwro2n5PkTBh6GEYi2afuCIQ8mjXAUdvEDg3um2GjJKtA%3d%3d'
  timestamp_decimal='635655897361394523'
  url=\
'https://contoso.com/wopi/files/JIB9h+LJpZWBDwvoIiQ5p3115zJWDecpGF9aCm1vOa5UMllgC7w?access_token=RLoY%2f3D73%2fjwt6IQqR1wHqCEKDxRf2v0GPDa0ZHTlA6ik1%2fNBHDD6bHCI0BQrvacjNBL8ok%2fZsVPI%2beAIA5mHSOUbhW9ohowwD6Ljlwro2n5PkTBh6GEYi2afuCIQ8mjXAUdvEDg3um2GjJKtA%3d%3d'
  proof_signature_base_64=\
'x0IeSOjUQNH2pvjMPkP4Jotzs5Weeqms4AlPxMQ5CipssUJbyKFjLWnwPg1Ac0XtSTiPD177BmQ1+KtmYvDTWQ1FmBuvpvKZDSKzXoT6Qj4LCYYQ0TxnN/OT231+qd50sOD8zAxhfXP56qND9tj5xqoHMa+lbuvNCqiOBTZw5f/dklSK7Wgdx7ST3Dq6S9xxDUfsLC4Tjq+EsvcdSNIWL/W6NRZdyWqlgRgE6X8t/2iyyMypURdOW2Rztc6w/iYhbuh22Ul6Jfu14KaDo6YkvBr8iHlK4CcQST9i0u044y1Jnh34UK4EPdVRZrvTmeJ/5DFLWOqEwvBlW2bpoYF+9A=='
}

# COMMENT OUT THIS LINE TO DISABLE EXAMPLE DATA
use_example_data


if [ -z "${proof_signature_base_64}" ] || [ -n "${7}" ]; then
  echo 'usage: wopi-proof-verify.bash MODULUS_BASE_64 EXPONENT_BASE6_4 TOKEN TIMESTAMP URL PROOF_SIGNATURE_BASE_64'
  echo
  echo '  URL must be the full URL as send by the HTTP client. Example: http(s)://HOSTNAME/foo?bar=baz'
  echo
  echo '  Creates "wopi-proof-verify_workdir/" in the current directory.'
  exit 1
fi

if [ -a "wopi-proof-verify_workdir" ]; then
  echo '"wopi-proof-verify_workdir/" already exists in the current directory.'
  echo 'Please delete manually before running the script again.'
  exit 1
fi

mkdir wopi-proof-verify_workdir/
cd wopi-proof-verify_workdir/

modulus_hex="$(echo -n "${modulus_base_64}" | base64 -d | xxd -plain -cols 0)"
exponent_hex="$(echo -n "${exponent_base_64}" | base64 -d | xxd -plain -cols 0)"

echo '
asn1=SEQUENCE:pubkeyinfo
[pubkeyinfo]
algorithm=SEQUENCE:rsa_alg
pubkey=BITWRAP,SEQUENCE:rsapubkey
[rsa_alg]
algorithm=OID:rsaEncryption
parameter=NULL
[rsapubkey] 
n=INTEGER:0x'"${modulus_hex}"'
e=INTEGER:0x'"${exponent_hex}" >> pubkey.asn1

openssl asn1parse -genconf pubkey.asn1 -out pubkey.der -noout
openssl rsa -in pubkey.der -inform der -pubin -out pubkey.pem 2>/dev/null

token_len=${#token}
url_len=${#url}

echo -n 'timestamp is: '
date --utc -d "@$(("${timestamp_decimal}" / 10000000 - 62135596800))"
# 62135596800: seconds since the year 1 A.D., more exactly 0001-01-01T00:00

# Can't properly store binary in variables.
(
  printf '%08X' "${token_len}" | xxd -r -p
  echo -n "${token}"
  printf '%08X' "${url_len}" | xxd -r -p
  echo -n "${url}" | tr '[:lower:]' '[:upper:]'
  echo -en '\x00\x00\x00\x08'  # timestamp binary length (always "8")
  printf '%016X' "${timestamp_decimal}" | xxd -r -p
) > signed_data.bin

echo -n "${proof_signature_base_64}" | base64 -d > proof_signature.bin

echo
set +e
echo 'openssl says:'
openssl dgst -verify pubkey.pem -keyform PEM -sha256 -signature proof_signature.bin -binary signed_data.bin
result="$?"
echo

if [ "${result}" -eq 0 ]; then
  echo 'Signature successfully verified!'
else
  echo 'Signature verification failed!'
  exit "${result}";
fi
