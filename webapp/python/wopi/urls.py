
from django.urls import path

from . import views

urlpatterns = [
    path('files/<int:file_id>', views.check_file_info, name='file_info'),
    path('files/<int:file_id>/contents', views.FileContentView.as_view(), name='file_content'),
]
