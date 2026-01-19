import React from 'react';
import './App.css';
import ServerAddressForm from './ServerAddressForm'
import LoaderForm from './LoaderForm'

class App extends React.Component<any, any> {
    constructor(props: any) {
        super(props);
        this.handleInputChanged = this.handleInputChanged.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.state = {
            serverAddress: '',
            startLoading: false,
            wopiUrl: '',
            token: ''
        };
    }
    handleInputChanged(address: string) {
         this.setState({serverAddress: address});
    }

    handleSubmit() {
        const locationOrigin = window.location.href.substring(0, window.location.href.lastIndexOf('/'));
        const scheme = window.location.protocol;  // http: | https:

        const wopiClientHost = this.state.serverAddress;
        if (!wopiClientHost) {
            alert('No server address entered');
            return;
        }
        if (!wopiClientHost.startsWith('http')) {
            alert('Warning! You have to specify the scheme protocol too (http|https) for the server address.')
            return;
        }
        if (!wopiClientHost.startsWith(scheme + '//')) {
            alert('Collabora Online server address scheme does not match the current page url scheme');
            return;
        }

        const wopiSrc =  `${locationOrigin}/wopi/files/1`;

        fetch(`/collaboraUrl?server=${wopiClientHost}`)
            .then(response => response.json())
            .then(data => {
                const wopiClientUrl = data.url
                const accessToken = data.token;
                const wopiUrl = `${wopiClientUrl}WOPISrc=${encodeURIComponent(wopiSrc)}`;
                console.log(`wopiUrl: ${wopiUrl}`)
                this.setState({
                    startLoading: true,
                    wopiUrl: wopiUrl,
                    token: accessToken
                })
            })
    }

    componentDidUpdate() {
        if (this.state.startLoading) {
            this.setState({startLoading: false})
        }
    }

    render() {
        let loaderForm;
        if (this.state.startLoading) {
            loaderForm = <LoaderForm
                url={this.state.wopiUrl}
                token={this.state.token}
            />
        }

        return (
            <div className="App">
                <ServerAddressForm
                    address={this.state.serverAddress}
                    onChange={this.handleInputChanged}
                    onSubmit={this.handleSubmit}/>
                {loaderForm}
                <iframe title="Collabora Online Viewer" id="collabora-online-viewer" name="collabora-online-viewer" allow="clipboard-read *; clipboard-write *; fullscreen *">
                </iframe>
            </div>
        );
    }
}

export default App;
