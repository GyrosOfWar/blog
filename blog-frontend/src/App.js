import React, { Component } from 'react'
import logo from './logo.svg'
import $ from 'jquery'
import { Button, Form, FormGroup, Label, Input, FormText } from 'reactstrap';

class BlogEntry extends Component {
    render() {
        const entry = this.props.entry
        const markup = { __html: entry.content }
        return (
            <div className="blog-entry" dangerouslySetInnerHTML={markup}>
            </div>
        )
    }
}

const LoginForm = React.createClass({
    render: function () {
        return (
            <Form>
                <FormGroup>
                    <Label for="username">Username</Label>
                    <Input type="text" name="username" id="username" />
                </FormGroup>
                <FormGroup>
                    <Label for="password">Password</Label>
                    <Input type="password" name="password" id="passwod" />
                </FormGroup>
            </Form>
        );
    }
})

const App = React.createClass({
    getPosts: function () {
        $.ajax({
            url: `/api/user/${this.state.user.id}/post`,
            method: "GET",
            dataType: "json",
            success: response => {
                this.setState({
                    blogPosts: response.result
                })
            }
        })
    },

    getUser: function () {
        console.log("Getting user!")
    },

    getInitialState: function () {
        return {
            blogPosts: [],
            user: null
        }
    },

    componentDidMount: function () {
        // this.getPosts()
    },

    render: function () {
        if (this.state.user == null) {
            return (<LoginForm />)
        } else {
            const items = this.state.blogPosts.map(function (entry) {
                return (<BlogEntry entry={entry} user={this.state.user} />)
            })

            return (
                <div className="content">
                    <h1>Blog!</h1>
                    <div className="blog-entries">
                        {items}
                    </div>
                </div>
            )
        }
    }
})

export default App
