from django.http import JsonResponse, HttpResponse, HttpResponseNotFound
from django.views.decorators.http import require_GET
from django.views import View

#  wopi CheckFileInfo endpoint
#
#  Returns info about the file with the given document id.
#  The response has to be in JSON format and at a minimum it needs to include
#  the file name and the file size.
#  The CheckFileInfo wopi endpoint is triggered by a GET request at
#  https://HOSTNAME/wopi/files/<document_id>
@require_GET
def check_file_info(request, file_id):
    print(f"CheckFileInfo: file id: {file_id}, access token: {request.GET.get('access_token')}")
    # test.txt is just a fake text file
    # the Size property is the length of the string
    # returned by the wopi GetFile endpoint
    res = {
        'BaseFileName': 'test.txt',
        'Size': 11,
        'UserId': 1,
        'UserCanWrite': True,
    }
    return JsonResponse(res)


class FileContentView(View):

    #  wopi GetFile endpoint
    #
    #  Given a request access token and a document id, sends back the contents of the file.
    #  The GetFile wopi endpoint is triggered by a request with a GET verb at
    #  https://HOSTNAME/wopi/files/<document_id>/contents
    @staticmethod
    def get(request, file_id):
        print(f"GetFile: file id: {file_id}, access token: {request.GET.get('access_token')}")
        # we just return the content of a fake text file
        # in a real case you should use the file id
        # for retrieving the file from the storage and
        # send back the file content as response
        content = 'Hello world!'
        return HttpResponse(content)

    #  wopi PutFile endpoint
    #
    #  Given a request access token and a document id, replaces the files with the POST request body.
    #  The PutFile wopi endpoint is triggered by a request with a POST verb at
    #  https://HOSTNAME/wopi/files/<document_id>/contents
    @staticmethod
    def post(request, file_id):
        print(f"PutFile: file id: {file_id}, access token: {request.GET.get('access_token')}")
        if not request.body:
            return HttpResponseNotFound('Not possible to get the file content.')
        content = request.read()
        # log to the console the content of the received file
        print(content.decode("utf-8-sig"))
        return HttpResponse()  # status 200
