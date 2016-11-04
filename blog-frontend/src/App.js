import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import $ from 'jquery';

class BlogEntry extends Component {
    render() {
        const entry = this.props.entry;
        const markup = {__html: entry.content};
        return (
            <div className="blog-entry" dangerouslySetInnerHTML={markup}>
            </div>
        );
    }
}

const App = React.createClass({
    getPosts: function () {
        $.ajax({
            url: "/api/user/0/post",
            method: "GET",
            dataType: "json",
            success: response => {
                this.setState({
                    blogPosts: response.result
                });
            }            
        });
    },
    
    getInitialState: function () {
        return {
            blogPosts: []
        };
    },

    componentDidMount: function () {
        this.getPosts();
    },
    
    render: function () {
        const items = this.state.blogPosts.map(function (entry) {
            return (<BlogEntry entry={entry} />);
        });
        
        return (
            <div className="content">
              <h1>Blog!</h1>
            <div className="blog-entries">
              {items}
            </div>
            </div>
        );
    }
});

export default App;
