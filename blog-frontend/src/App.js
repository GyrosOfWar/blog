import React, { Component } from 'react'
import $ from 'jquery'
import { FormGroup, ControlLabel, FormControl, Button } from 'react-bootstrap'
import { Router, Route, Link, browserHistory } from 'react-router'
const jwtDecode = require('jwt-decode')

class BlogListEntry extends Component {
  render() {
    const entry = this.props.entry
    return (
      <li>{entry.title}</li>
    )
  }
}

const RegisterForm = React.createClass({
  register: function () {

  },

  render: function () {
    return (<div className="form">
      <h1>Register</h1>
      <FormGroup controlId="username">
        <ControlLabel>Username</ControlLabel>
        <FormControl type="text" id="username" />
      </FormGroup>
      <FormGroup controlId="password">
        <ControlLabel>Password</ControlLabel>
        <FormControl type="password" id="password" />
      </FormGroup>
      <Button bsStyle="primary" id="register-button" onClick={this.register}>Register</Button>
    </div>
    )
  }
})

const LoginForm = React.createClass({
  render: function () {
    return (
      <div className="form">
        <h1>Login</h1>
        <FormGroup controlId="username">
          <ControlLabel>Username</ControlLabel>
          <FormControl type="text" id="username" />
        </FormGroup>

        <FormGroup controlId="password">
          <ControlLabel>Password</ControlLabel>
          <FormControl type="password" id="password" />
        </FormGroup>
        <Button bsStyle="primary" id="login-button" onClick={this.login}>Login</Button>
      </div>
    )
  },

  login: function () {
    const loginData = {
      name: $("#username").val(),
      password: $("#password").val()
    }

    $.ajax({
      url: "/api/token",
      method: "POST",
      dataType: "json",
      data: JSON.stringify(loginData),
      success: response => {
        const token = response.result
        localStorage.setItem('jwt', token)
        const parsed = jwtDecode(token)
        this.props.loginSuccessfulCallback(parsed);
      }
    })
  }
})

const NotFound = React.createClass({
  render: function () {
    return (<h1>404 Not Found</h1>)
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

  logout: function () {
    localStorage.removeItem('jwt')
    this.setState({
      user: null
    })
  },

  getUser: function (userId) {
    $.ajax({
      url: `/api/user/${userId}`,
      method: "GET",
      dataType: "json",
      beforeSend: function (req) {
        const token = localStorage.getItem('jwt')
        req.setRequestHeader("Authorization", `Bearer ${token}`)
      },
      success: resp => {
        this.setState({
          user: resp.result
        })
        this.getPosts()
      }
    })
  },

  getInitialState: function () {
    return {
      blogPosts: [],
      user: null
    }
  },

  hasToken: function () {
    return !!localStorage.getItem('jwt')
  },

  loginSuccess: function (jwt) {
    this.getUser(jwt.sub)
  },

  render: function () {
    let inner = null

    if (!this.hasToken()) {
      inner = <LoginForm loginSuccessfulCallback={this.loginSuccess} />
    } else {
      if (this.state.user == null) {
        const jwt = jwtDecode(localStorage.getItem('jwt'))
        this.getUser(jwt.sub)
      }
      const user = this.state.user
      const items = this.state.blogPosts.map(function (entry) {
        return (<BlogListEntry entry={entry} user={user} key={entry.id} />)
      })

      inner = (
        <div className="content">
          <h1>Blog!</h1>
          <ul className="blog-entries">
            {items}
          </ul>
        </div>
      )
    }
    return (
      <Router>
        <Route path="/" component={App}>
          <Route path="login" component="LoginForm" />
          <Route path="register" component="RegisterForm" />
        </Route>
        <Route path="*" component={NotFound} />
      </Router>
    )
  }
})

export default App
