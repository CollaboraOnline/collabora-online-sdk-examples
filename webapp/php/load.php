<?php

$errorMsg = [
    101 => 'GET Request not found',
    201 => 'Collabora Online server address is not valid',
    202 => 'Collabora Online server address scheme does not match the current page url scheme',
    203 => 'No able to retrieve the discovery.xml file from the Collabora Online server with the submitted address.',
    102 => 'The retrieved discovery.xml file is not a valid XML file',
    103 => 'The requested mime type is not handled',
    204 => 'Warning! You have to specify the scheme protocol too (http|https) for the server address.'
];

function getDiscovery($server) {
    $discoveryUrl = $server.'/hosting/discovery';
    $res = file_get_contents($discoveryUrl);
    return $res;
}

function getWopiSrcUrl($discovery_parsed, $mimetype) {
    if ($discovery_parsed === null || $discovery_parsed == false) {
        return null;
    }
    $result = $discovery_parsed->xpath(sprintf('/wopi-discovery/net-zone/app[@name=\'%s\']/action', $mimetype));
    if ($result && count($result) > 0) {
        return $result[0]['urlsrc'];
    }
    return null;
}

function strStartsWith($s, $ss) {
    $res = strrpos($s, $ss);
    return !is_bool($res) && $res == 0;
}

$_WOPI_SRC = '';
$_HOST_SCHEME = isset($_SERVER['HTTPS']) ? 'https' : 'http';

function main() {
    global $_WOPI_SRC;
    global $_HOST_SCHEME;

    if (!isset($_GET['submit'])) {
        return 101;
    }
    $wopiClientServer = $_GET['collabora-online-server'];
    if (!$wopiClientServer) {
        return 201;
    }
    $wopiClientServer = trim($wopiClientServer);

    if (!strStartsWith($wopiClientServer, 'http')) {
        return 204;
    }


    if (!strStartsWith($wopiClientServer, $_HOST_SCHEME . '://')) {
        return 202;
    }

    $discovery = getDiscovery($wopiClientServer);
    if (!$discovery) {
        return 203;
    }

    $loadEntities = libxml_disable_entity_loader(true);
    $discovery_parsed = simplexml_load_string($discovery);
    libxml_disable_entity_loader($loadEntities);
    if (!$discovery_parsed) {
        return 102;
    }

    $_WOPI_SRC = getWopiSrcUrl($discovery_parsed, 'text/plain');
    if (!$_WOPI_SRC) {
        return 103;
    }

    return 0;
}

$errorCode = main();

?>

<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html lang="en">
<body>
<div style="display: none">
    <form action="" enctype="multipart/form-data" method="post" target="_self" id="collabora-submit-form">
        <input name="access_token" value="test" type="hidden" />
        <input type="submit" value="" />
    </form>
</div>
<div>
    <p> Something went wrong :-( </p>
    <p> <?php
        if ($errorCode > 200)
            echo $errorMsg[$errorCode];
        ?>
    </p>
</div>

<script type="text/ecmascript">
    function loadDocument() {
        var wopiSrc = window.location.origin + '/example_php/wopi/files/1';

        var wopiClientUrl = "<?php echo $_WOPI_SRC; ?>";
        if (!wopiClientUrl) {
            console.log('error: wopi client url not found');
            return;
        }

        var wopiUrl = wopiClientUrl + 'WOPISrc=' + wopiSrc;

        var formElem = document.getElementById("collabora-submit-form");
        if (!formElem) {
            console.log('error: submit form not found');
            return;
        }
        formElem.action = wopiUrl;
        formElem.submit();
    }

    loadDocument();
</script>
</body>
</html>
