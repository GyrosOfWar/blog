import * as React from 'react';
import "simplemde";

export class BlogEditor extends React.Component<any, any> {
    editor: SimpleMDE;

    constructor(props: any) {
        super(props);
        this.editor = new SimpleMDE();
    }
    
    render(): JSX.Element {
        return <input type="textarea" id="editor" />
    }
}