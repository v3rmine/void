#!/usr/bin/ruby
#script by Blackksoulls for easy setup of Ebooks bot (http://wikikuro.herokuapp.com/index.php?title=Ebooks_bot)
require 'io/console'

#Definition des variables customs
$modelname = "tweets"

def cls #Pour clear l'invite de commande
	system "clear" or system "cls"
end

def press #Pour le wait until press                                                                                
  	print "Appuyez sur une touche pour continuer..."                                                                                                    
  	STDIN.getch                                                                                                              
  	print "            \r" # extra space to overwrite in case next sentence is short                                                                                                              
  	cls
end  

def setup #Préparation du terrain
	sleep(2)
	puts "\n"
	sleep(0.07)
	puts "██╗  ██╗██╗   ██╗██████╗  ██████╗ "
	sleep(0.07)
	puts "██║ ██╔╝██║   ██║██╔══██╗██╔═══██╗"
	sleep(0.07)
	puts "█████╔╝ ██║   ██║██████╔╝██║   ██║"
	sleep(0.07)
	puts "██╔═██╗ ██║   ██║██╔══██╗██║   ██║"
	sleep(0.07)
	puts "██║  ██╗╚██████╔╝██║  ██║╚██████╔╝"
	sleep(0.07)
	puts "╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ "
	sleep(0.07)
	puts ""

	sleep(0.75)
	puts "Bienvenues dans le EESS, le Easy Ebooks Setup Script :D"
	sleep(2)
	puts ""
	puts "Je vais vous demandez quelques petites infos pour"
	puts "la configuration de votre bot"
	sleep(2)
	puts ""
	puts "Attention ce script à été fait pour fonctionner pour"
	puts "le tuto que j'ai rédigé ! Il peux donc avoir des"
	puts "disfonctionnements si le tuto n'a pas été bien suivi."
	sleep(2)
	puts ""
	press
end

def basicsquestions #Un peu de customisation
	puts "Le nom de votre compte principal: "
	$name = gets.chomp
	puts "Le pseudo du compte twitter de votre bot sans le @: "
	$pseudo = gets.chomp
	cls
end

def keys #On recupère les clées
	puts "Il est temps de ressortir le fichier texte avec les clées"
	puts ""
	puts "Consumer Key: "
	$conskey = gets.chomp
	puts "Consumer Secret: "
	$conssecret = gets.chomp
	puts "Access Token: "
	$accesstoken = gets.chomp
	puts "Access Token Secret: "
	$accesstokensecret = gets.chomp
end

def create #fichier de base
  File.open("bots.rb", "w+") {|f| f.write("require 'twitter_ebooks'
include Ebooks

CONSUMER_KEY = '#{$conskey}'  # Your app consumer key
CONSUMER_SECRET = '#{$conssecret}'  # Your app consumer secret
OAUTH_TOKEN = '#{$accesstoken}'  # Token connecting the app to this account
OAUTH_TOKEN_SECRET = '#{$accesstokensecret}'  # Secret connecting the app to this account

ROBOT_ID = 'ebooks' # leave this as ebooks. Prefer not to talk to other robots
MODEL_NAME = 'tweets' # the name of your model
TWITTER_USERNAME = '#{$pseudo}' # Ebooks account username

# Information about a particular Twitter user we know
class UserInfo
  attr_reader :username

  # @return [Integer] how many times we can pester this user unprompted
  attr_accessor :pesters_left

  # @param username [String]
  def initialize(username)
    @username = username
    @pesters_left = 1
  end
end

class MyBot < Ebooks::Bot
  attr_accessor :original, :model, :model_path
  # Configuration here applies to all MyBots
  def configure
    # Consumer details come from registering an app at https://dev.twitter.com/
    # Once you have consumer details, use 'ebooks auth' for new access tokens
    self.consumer_key = CONSUMER_KEY # Your app consumer key
    self.consumer_secret = CONSUMER_SECRET # Your app consumer secret

    # Users to block instead of interacting with
    #self.blacklist = ['tnietzschequote']

    # Range in seconds to randomize delay when bot.delay is called
    self.delay_range = 1..6
    @userinfo = {}
  end

  def top100; @top100 ||= model.keywords.take(100); end
  def top20;  @top20  ||= model.keywords.take(20); end

  def on_startup
    load_model!
    scheduler.every '6h' do
      # Tweet something every 6 hours
      # See https://github.com/jmettraux/rufus-scheduler
      # tweet('hi')
      # pictweet('hi', 'cuteselfie.jpg')
      tweet(model.make_statement)
    end
  end

  def on_message(dm)
    # Reply to a DM
    delay do
      # reply(dm, 'secret secrets')
      reply(dm, model.make_response(dm.text))
    end
  end

  def on_mention(tweet)
    # Become more inclined to pester a user when they talk to us
    userinfo(tweet.user.screen_name).pesters_left += 1
    # Reply to a mention
    delay do
      # reply(tweet, 'oh hullo')
      reply(tweet, model.make_response(meta(tweet).mentionless, meta(tweet).limit))
    end
  end

  def on_timeline(tweet)
    # Reply to a tweet in the bot's timeline
    # reply(tweet, 'nice tweet')
    return if tweet.retweeted_status?
    return unless can_pester?(tweet.user.screen_name)

    tokens = Ebooks::NLP.tokenize(tweet.text)

    interesting = tokens.find { |t| top100.include?(t.downcase) }
    very_interesting = tokens.find_all { |t| top20.include?(t.downcase) }.length > 2

    delay do
      if very_interesting
        favorite(tweet) if rand < 0.5
        retweet(tweet) if rand < 0.1
        if rand < 0.01
          userinfo(tweet.user.screen_name).pesters_left -= 1
          reply(tweet, model.make_response(meta(tweet).mentionless, meta(tweet).limit))
        end
      elsif interesting
        favorite(tweet) if rand < 0.05
        if rand < 0.001
          userinfo(tweet.user.screen_name).pesters_left -= 1
          reply(tweet, model.make_response(meta(tweet).mentionless, meta(tweet).limit))
        end
      end
    end
  end

  # Find information we've collected about a user
  # @param username [String]
  # @return [Ebooks::UserInfo]
  def userinfo(username)
    @userinfo[username] ||= UserInfo.new(username)
  end

  # Check if we're allowed to send unprompted tweets to a user
  # @param username [String]
  # @return [Boolean]
  def can_pester?(username)
    userinfo(username).pesters_left > 0
  end

  # Only follow our original user or people who are following our original user
  # @param user [Twitter::User]
  def can_follow?(username)
    @original.nil? || username == @original || twitter.friendship?(username, @original)
  end

  def on_follow(user)
    #if can_follow?(user.screen_name) # Si il suit votre main account votre bot vas le follow
      follow(user.screen_name)
    #end
  end

  def on_unfollow(user)
      twitter.unfollow(tweet.user.screen_name)
  end

  def on_favorite(user, tweet)
    # Follow user who just favorited bot's tweet
    # follow(user.screen_name)
  end

  def on_retweet(tweet)
    # Follow user who just retweeted bot's tweet
    # follow(tweet.user.screen_name)
  end

  private
  def load_model!
    return if @model

    @model_path ||= 'model/#{$MODEL_NAME}.model'

    log 'Loading model' + model_path}
    @model = Ebooks::Model.load(model_path)
  end
end

# Make a MyBot and attach it to an account
MyBot.new(TWITTER_USERNAME) do |bot|
  bot.access_token = OAUTH_TOKEN # Token connecting the app to this account
  bot.access_token_secret = OAUTH_TOKEN_SECRET # Secret connecting the app to this account
  bot.original = '#{$name}' # Nom de votre compte principal
end") }
end

def config
	if File.exist?("bots.rb") == false
		create
	else
		File.rename("bots.rb", "old.bots.rb")
		create
	end
	cls
end

setup
basicsquestions
keys
config
puts "Et voilà vous pouvez maintenant lancer votre bot"
puts "avec la commande 'ebooks start'"
sleep(2)
puts ""
press
exit()
