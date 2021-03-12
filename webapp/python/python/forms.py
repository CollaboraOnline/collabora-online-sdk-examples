from django import forms
from django.core.exceptions import ValidationError
from django.forms.utils import ErrorList


class CollaboraOnlineServerForm(forms.Form):
    collabora_online_server = forms.CharField(label='Collabora Online Server')

    def __init__(self, data=None, files=None, auto_id='id_%s', prefix=None,
                 initial=None, error_class=ErrorList, label_suffix=None,
                 empty_permitted=False, field_order=None, use_required_attribute=None, renderer=None):
        super().__init__(data, files, auto_id, prefix, initial, error_class, label_suffix, empty_permitted, field_order,
                         use_required_attribute, renderer)
        self.scheme = 'http'

    def set_scheme(self, scheme):
        self.scheme = scheme

    def clean_collabora_online_server(self):
        url = self.cleaned_data['collabora_online_server']
        if not url.startswith('http'):
            raise ValidationError("Warning! You have to specify the scheme protocol too (http|https) for the server "
                                  "address.")
        if not url.startswith(f'{self.scheme}://'):
            raise ValidationError('Collabora Online server address scheme does not match the current page url scheme')

        return url
