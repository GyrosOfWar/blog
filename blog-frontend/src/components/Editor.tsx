import * as React from 'react';

export class BlogEditor extends React.Component<any, any> {
    editor: SimpleMDE;

    constructor(props: any) {
        super(props);
        this.editor = new SimpleMDE({ element: document.getElementById("editor") as HTMLElement });
    }
    
    render(): JSX.Element {
        return <input type="textarea" id="editor" />
    }
}