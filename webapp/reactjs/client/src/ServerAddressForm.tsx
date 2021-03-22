import React from 'react';

class ServerAddressForm extends React.Component<any, any> {
    constructor(props: any) {
        super(props);
        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    handleChange(event: any) {
        this.props.onChange(event.target.value);
    }

    handleSubmit(event: any) {
        event.preventDefault();
        this.props.onSubmit();
    }

    render() {
        return (
            <form action="" method="get" onSubmit={this.handleSubmit}>
                <label htmlFor="collabora-online-server">
                    Collabora Online Server:
                    <input type="text" value={this.props.address} onChange={this.handleChange} />
                </label>
                <input type="submit" value="Load Collabora Online" />
            </form>
        );
    }
}

export default ServerAddressForm;
