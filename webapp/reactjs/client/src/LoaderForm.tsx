import React from 'react';

class LoaderForm extends React.Component<any, any> {
    private readonly formElem: React.RefObject<HTMLFormElement>;

    constructor(props: any) {
        super(props);
        this.formElem = React.createRef();
    }

    componentDidMount() {
        if (this.formElem.current)
            this.formElem.current.submit();
    }

    render() {
        return(
            <div style={{display: "none"}}>
                <form ref={this.formElem} action={this.props.url} encType="multipart/form-data" method="post" target="collabora-online-viewer"
                      id="collabora-submit-form">
                    <input name="access_token" value={this.props.token} type="hidden" id="access-token"/>
                    <input type="submit" value=""/>
                </form>
            </div>
        );
    }
}

export default LoaderForm;
