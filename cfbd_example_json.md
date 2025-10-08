** Games Response
```json
[
  {
    "id": 0,
    "season": 0,
    "week": 0,
    "seasonType": "regular",
    "startDate": "2025-09-22T22:18:00.030Z",
    "startTimeTBD": true,
    "completed": true,
    "neutralSite": true,
    "conferenceGame": true,
    "attendance": 0,
    "venueId": 0,
    "venue": "string",
    "homeId": 0,
    "homeTeam": "string",
    "homeConference": "string",
    "homeClassification": "fbs",
    "homePoints": 0,
    "homeLineScores": [
      0
    ],
    "homePostgameWinProbability": 0,
    "homePregameElo": 0,
    "homePostgameElo": 0,
    "awayId": 0,
    "awayTeam": "string",
    "awayConference": "string",
    "awayClassification": "fbs",
    "awayPoints": 0,
    "awayLineScores": [
      0
    ],
    "awayPostgameWinProbability": 0,
    "awayPregameElo": 0,
    "awayPostgameElo": 0,
    "excitementIndex": 0,
    "highlights": "string",
    "notes": "string"
  }
]
```

** Teams Response
```json
[
  {
    "id": 0,
    "school": "string",
    "mascot": "string",
    "abbreviation": "string",
    "alternateNames": [
      "string"
    ],
    "conference": "string",
    "division": "string",
    "classification": "string",
    "color": "string",
    "alternateColor": "string",
    "logos": [
      "string"
    ],
    "twitter": "string",
    "location": {
      "id": 0,
      "name": "string",
      "city": "string",
      "state": "string",
      "zip": "string",
      "countryCode": "string",
      "timezone": "string",
      "latitude": 0,
      "longitude": 0,
      "elevation": "string",
      "capacity": 0,
      "constructionYear": 0,
      "grass": true,
      "dome": true
    }
  }
]
```
** Stats Game Advanced Response (Game Advanced Stats)
```json
[
  {
    "gameId": 0,
    "season": 0,
    "seasonType": "allstar",
    "week": 0,
    "team": "string",
    "opponent": "string",
    "offense": {
      "passingPlays": {
        "explosiveness": 0,
        "successRate": 0,
        "totalPPA": 0,
        "ppa": 0
      },
      "rushingPlays": {
        "explosiveness": 0,
        "successRate": 0,
        "totalPPA": 0,
        "ppa": 0
      },
      "passingDowns": {
        "explosiveness": 0,
        "successRate": 0,
        "ppa": 0
      },
      "standardDowns": {
        "explosiveness": 0,
        "successRate": 0,
        "ppa": 0
      },
      "openFieldYardsTotal": 0,
      "openFieldYards": 0,
      "secondLevelYardsTotal": 0,
      "secondLevelYards": 0,
      "lineYardsTotal": 0,
      "lineYards": 0,
      "stuffRate": 0,
      "powerSuccess": 0,
      "explosiveness": 0,
      "successRate": 0,
      "totalPPA": 0,
      "ppa": 0,
      "drives": 0,
      "plays": 0
    },
    "defense": {
      "passingPlays": {
        "explosiveness": 0,
        "successRate": 0,
        "totalPPA": 0,
        "ppa": 0
      },
      "rushingPlays": {
        "explosiveness": 0,
        "successRate": 0,
        "totalPPA": 0,
        "ppa": 0
      },
      "passingDowns": {
        "explosiveness": 0,
        "successRate": 0,
        "ppa": 0
      },
      "standardDowns": {
        "explosiveness": 0,
        "successRate": 0,
        "ppa": 0
      },
      "openFieldYardsTotal": 0,
      "openFieldYards": 0,
      "secondLevelYardsTotal": 0,
      "secondLevelYards": 0,
      "lineYardsTotal": 0,
      "lineYards": 0,
      "stuffRate": 0,
      "powerSuccess": 0,
      "explosiveness": 0,
      "successRate": 0,
      "totalPPA": 0,
      "ppa": 0,
      "drives": 0,
      "plays": 0
    }
  }
]
```

** Drives Response
```json
[
  {
    "offense": "string",
    "offenseConference": "string",
    "defense": "string",
    "defenseConference": "string",
    "gameId": 0,
    "id": "string",
    "driveNumber": 0,
    "scoring": true,
    "startPeriod": 0,
    "startYardline": 0,
    "startYardsToGoal": 0,
    "startTime": {
      "seconds": 0,
      "minutes": 0
    },
    "endPeriod": 0,
    "endYardline": 0,
    "endYardsToGoal": 0,
    "endTime": {
      "seconds": 0,
      "minutes": 0
    },
    "elapsed": {
      "seconds": 0,
      "minutes": 0
    },
    "plays": 0,
    "yards": 0,
    "driveResult": "string",
    "isHomeOffense": true,
    "startOffenseScore": 0,
    "startDefenseScore": 0,
    "endOffenseScore": 0,
    "endDefenseScore": 0
  }
]
```
