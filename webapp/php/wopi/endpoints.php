<?php

    /**
     *  Parses the URL in order to route the data in the request to the appropriate wopi endpoint handler.
     */
    function parseWopiRequest($uri) {
        $path = parse_url($uri, PHP_URL_PATH);
        preg_match('/^\/example_php\/wopi\/files\/([[:alnum:]]+)(\/contents)?$/', $path, $matches);
        $numMatches = count($matches);

        // In a real case it is needed to encode an access token based on the file owner id
        // which should be used to set the value attribute of the 'access_token' <input> field
        // in the form used for requesting to load Collabora Online (see load.php).
        // In this example the access token attached to the request is retrieved and logged
        // to the Apache error log file. Anyway in a real implementation it has to be decoded
        // so that is possible to get the file owner id and check if he has really the rights
        // for the requested document.
        $token = getAccessToken($uri);
        error_log('INFO: access token: ' . $token);

        if ($numMatches == 3) {
            switch ($_SERVER['REQUEST_METHOD']) {
                case  'GET': wopiGetFile($matches[1]); break;
                case 'POST': wopiPutFile($matches[1]); break;
            }
        }
        else if ($numMatches == 2) {
            wopiCheckFileInfo($matches[1]);
        }
    }

    /**
     *  Parses the URL in order to extract from the query section the access token value.
     */
    function getAccessToken($uri) {
        $query = parse_url($uri, PHP_URL_QUERY);
        if ($query) {
            $params = explode('&', $query);
            foreach ($params as $param) {
                $pair = explode('=', $param);
                if ($pair && count($pair) == 2) {
                    if ($pair[0] == 'access_token') {
                        return $pair[1];
                    }
                }
            }
        }
        return '';
    }

    /**
     *  Returns info about the file with the given document id.
     *  The response has to be in JSON format and at a minimum it needs to include
     *  the file name and the file size.
     *  The CheckFileInfo wopi endpoint is triggered by a GET request at
     *  https://HOSTNAME/example_php/wopi/files/<document_id>
     */
    function wopiCheckFileInfo($documentId) {
        // test.txt is just a fake text file
        // the Size property is the length of the string
        // returned in wopiGetFile
        $response = [
            'BaseFileName' => 'test.txt',
            'Size' => 11,
            'UserId' => 1,
            'UserCanWrite' => true
        ];

        $jsonResponse = json_encode($response);

        echo $jsonResponse;
    }

    /* *
     *  Given a request access token and a document id, sends back the contents of the file.
     *  The GetFile wopi endpoint is triggered by a request with a GET verb at
     *  https://HOSTNAME/example_php/wopi/files/<document_id>/contents
     */
    function wopiGetFile($documentId) {
        // we just return the content of a fake text file
        // in a real case you should use the document id
        // for retrieving the file from the storage and
        // send back the file content as response
        $fileContent = 'Hello World!';
        echo $fileContent;
    }

    /**
     *  Given a request access token and a document id, replaces the files with the POST request body.
     *  The PutFile wopi endpoint is triggered by a request with a POST verb at
     *  https://HOSTNAME/example_php/wopi/files/<document_id>/contents
     */
    function wopiPutFile($documentId) {
        // we log to the apache error log file so that is possible
        // to check that saving has triggered this wopi endpoint
        error_log('INFO: wopiPutFile invoked: document id: ' . $documentId);

        $fh = fopen('php://input', 'r');
        $fileContent = '';
        while ($line = fgets($fh)) {
            $fileContent = $fileContent . $line;
        }
        fclose($fh);
        // you can check the new file content on the apache error log file
        error_log('INFO: ' . $fileContent);
    }

    $requestUri = $_SERVER['REQUEST_URI'];
    parseWopiRequest($requestUri);
