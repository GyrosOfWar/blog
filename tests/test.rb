require 'rest-client'
require 'json'

# TODO maybe write this in Rust too

API = 'http://localhost:5000/api'
TEST_POST = {
  title: "Ruby is okay",
  content: "# Ruby is okay\n* Some stuff is bad.\n* Some stuff is good.",
  tags: ["ruby", "programming", "opinion"],
  owner_id: 0
}.to_json

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

page = get_page()['result']
puts page.length



