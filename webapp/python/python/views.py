import os
from django.shortcuts import render
import requests
from lxml import etree
from .forms import CollaboraOnlineServerForm
import urllib.parse


def index(request):
    scheme = request.scheme
    host = request.get_host()
    wopi_src = urllib.parse.quote_plus(f'{scheme}://{host}/wopi/files/1')
    print(wopi_src)
    wopi_url = ''
    if request.method == 'POST':
        form = CollaboraOnlineServerForm(request.POST)
        form.set_scheme(scheme)
        if form.is_valid():
            wopi_client_host = form.cleaned_data['collabora_online_server']
            print(wopi_client_host)
            wopi_client_url = get_collabora_url(wopi_client_host, 'text/plain')
            if wopi_client_url:
                wopi_url = wopi_client_url + 'WOPISrc=' + wopi_src
            print(f'wopi client url: {wopi_url}')
    else:
        form = CollaboraOnlineServerForm(initial={'collabora_online_server': ''})
        form.set_scheme(scheme)

    context = {
        'form': form,
        'wopi_url': wopi_url,
        'access_token': 'test',
    }
    return render(request, 'index.html', context)


def get_collabora_url(server, mime_type):
    #
    # WARNING: `disable_verify_cert` should never be `True` on a production server.
    # This is only done to allow the use of self signed certificates on the Collabora
    # Online server for example purpose.
    #
    ##### TODO looks good, maybe use comment in other examples!
    disable_verify_cert = 'DISABLE_TLS_CERT_VALIDATION' in os.environ and os.environ['DISABLE_TLS_CERT_VALIDATION']
    response = requests.get(server + '/hosting/discovery', verify=not disable_verify_cert)
    discovery = response.text
    if not discovery:
        print('No able to retrieve the discovery.xml file from the Collabora Online server with the submitted address.')
        return
    # print(discovery)
    parsed = etree.fromstring(discovery)
    if parsed is None:
        print('The retrieved discovery.xml file is not a valid XML file')
        return
    result = parsed.xpath(f"/wopi-discovery/net-zone/app[@name='{mime_type}']/action")
    if len(result) != 1:
        print('The requested mime type is not handled')
        return
    online_url = result[0].get('urlsrc')
    print('online url: ' + online_url)
    return online_url
