Please see [../README.md](../README.md) for usage and general information.

## Requirements

- This example requires [Python][] version 3.6 or higher
- You also need to install with [pip][] the following packages:
  * [Django][] 3.1.7
  * requests 2.25.1
  * lxml 4.6.2
- For SSL support you need in addition:
  * django-extensions
  * Werkzeug 2.0 (later versions are incompatible)
  * pyOpenSSL
- These are only the requirements to run this example as-is.

## Configuration

1. Clone this repository
2. Create a local vistual environment on the `python` project's folder:
   * create folder `mkdir venv`
   * create environment `python3 -m venv ./venv`
   * activate it `source ./venv/bin/activate`
3. Install required package dependencies: `pip install -r requirements.txt`

## Run the example

1. Start the Django development server: `python manage.py runserver 0.0.0.0:8000`
2. Start your browser and make it point to the url `http://<host>:8000`

Or with HTTPS after configuring the certificates (see below)

1. Start the Django development server: `run_https.sh`
2. Start your browser and make it point to the url `https://<host>:8000`

   To make it reachable by the Collabora Online server use as `<host>` the IP address of the machine where the Django
   server is running. In case the Django server can't be reached you could also need to open the port 8000 on the firewall.

   When both the Collabora Online server and the Django server belong to the same LAN `<host>` should be a local IP address.

   In case the Collabora Online server is on a remote host on the internet, `<host>` must be the public IP address of
   the machine where the Django example is running.
3. In the form text field enter the address for your Collabora Online server including the protocol scheme,
   then click on the `Load Collabora Online` button.
4. An instance of Collabora Online should appear into an iframe embedded in the same page,
   showing a text document loaded with content a simple `Hello world!` text line.
5. Try to type in some more text ( e.g. `Hi!`) and then perform a `Save` action.
   In the console where the Django server is running you should get the following output:
   * `PutFile: file id: 1`  - the PutFile wopi endpoint has been triggered
   * `Hello World! Hi!` - the updated file content has been successfully received

## Certificates

It is highly recommended to setup TLS certificates for https.

If you don't have a key pair, I recommend using
[minica](https://github.com/jsha/minica) to generate a self-signed
one.

**THIS IS ONLY FOR TEST AND DEVELOPMENT. NEVER USE SELF SIGNED
CERTIFICATE IN A PRODUCTION ENVIRONMENT**

Then set the environment to indicate where to load the certificate from.

- `SSL_KEY_FILE` contains the path to the private key. If you used
  the `minica` tool mentionned above, it's the path to the
  `minica-key.pem` file.
- `SSL_CRT_FILE` contains the path to the public certificate. If you used
  the `minica` tool mentionned above, it's the path to the
  `minica.pem` file.
- `DISABLE_TLS_CERT_VALIDATION` set to `1` if you use certificates that do
  not validate on the Collabora Online server. **NEVER TO BE USED IN PRODUCTION**

---

[Python]: https://docs.python.org/3/
[pip]: https://pypi.org/project/pip/
[Django]: https://docs.djangoproject.com
