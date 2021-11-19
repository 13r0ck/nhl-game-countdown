# Unofficial NHL Game Time Countdown
Choose any Team!
Always know how long until the start of your team's next game!


## Build for [LaMetric](https://lametric.com)!



## 100% Open Source
All of the code here is what runs on my server. There is no tracking and I log **none** of the data. If you would like to run your own server the docker file is provided, and can be built as follows.
```bash
sudo docker build -t nhl-game-countdown . && sudo docker run --env PORT='8080' -p 8080:8080 nhl-game-countdown
```

The API is very simple. Just create a `GET` request to `https://nhl.gtcountdown.com/?team=<team_name>`

For example: `https://nhl.gtcountdown.com/?team=Colorado+Avalanche`
The `team` names are all in plain engligh, but the full list is provided below.

```
Carolina Hurricanes
Columbus Blue Jackets
New Jersey Devils
New York Islanders
New York Rangers
Philadelphia Flyers
Pittsburgh Penguins
Washington Capitals
Boston Bruins
Buffalo Sabres
Detroit Red Wings
Florida Panthers
Montréal Canadiens
Ottawa Senators
Tampa Bay Lightning
Toronto Maple Leafs
Arizona Coyotes
Chicago Blackhawks
Colorado Avalanche
Dallas Stars
Minnesota Wild
Nashville Predators
St. Louis Blues
Winnipeg Jets
Anaheim Ducks
Calgary Flames
Edmonton Oilers
Los Angeles Kings
San Jose Sharks
Seattle Kraken
Vancouver Canucks
Vegas Golden Knights
```