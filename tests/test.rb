require 'rest-client'
require 'json'

API = 'http://localhost:5000/api'
TEST_POST = {
  title: "Ruby is okay",
  content: "* Some stuff is bad.\n* Some stuff is good.",
  tags: ["ruby", "programming", "opinion"],
  owner_id: 0,
  published: true
}.to_json

def register_user(username, password)
    req = RestClient.post "#{API}/user", {name: username, password: password}.to_json
    JSON.parse(req.body)['result']
end

def get_token(username, password)
  req = RestClient.post "#{API}/token", {name: username, password: password}.to_json
  JSON.parse(req.body)['result']
end

def add_post(token)
  req = RestClient.post "#{API}/user/0/post", TEST_POST, {:Authorization => "Bearer #{token}"}
  JSON.parse(req.body)
end

def get_page()
  req = RestClient.get "#{API}/user/0/post?limit=25&offset=0"
  JSON.parse(req.body)
end

puts register_user('martin', 'martin4817')
puts add_post(get_token('martin','martin4817'))
