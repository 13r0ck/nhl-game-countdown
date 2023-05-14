# Unofficial NHL Game Time Countdown
Choose any Team!
Always know how long until the start of your team's next game!
When the date isn't known, your LaMetric will just act as a standard clock with your team's logo!


## Build for [LaMetric](https://lametric.com)!



## 100% Open Source
All of the code here is what runs on my server. There is no tracking and I log **none** of the data. If you would like to run your own server the docker file is provided, and can be built as follows.
```bash
sudo docker build -t nhl-game-countdown . && sudo docker run --env PORT='8080' -p 8080:8080 nhl-game-countdown
```

The API is very simple. Just create a `GET` request to `https://nhl.gtcountdown.com/?team=<team_name>&utc_offset=<local_utc_offset>&format=<use 24-hour?>`

For example: `https://nhl.gtcountdown.com/?team=Colorado+Avalanche&utc_offset=UTC-0700&format=0`
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
Colorado Avalanche // default on error
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

The UTC offset is for the error state of the timer. If the time until the next game isn't known, then we default to being a standard clock.
The UTC offset is how the server calculates the local time for the user. The format is "UTC(+/-)<Digits Of Offset>", all values are:
```
UTC-1200
UTC-1100
UTC-1000
UTC-0930
UTC-0900
UTC-0800
UTC-0700 // default on error
UTC-0600
UTC-0500
UTC-0400
UTC-0300
UTC-0330
UTC-0200
UTC-0100
UTC-0000
UTC+0100
UTC+0200
UTC+0300
UTC+0330
UTC+0400
UTC+0430
UTC+0500
UTC+0530
UTC+0545
UTC+0600
UTC+0700
UTC+0800
UTC+0900
UTC+0930
UTC+1000
UTC+1030
UTC+1100
UTC+1200
UTC+1245
UTC+1300
UTC+1400
```

Time format can also be specified. `format=0` is 12-hour, `format=1` is 24-hour.
